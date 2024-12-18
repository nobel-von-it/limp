use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use crate::error::LimpError;

const MAIN_SNIP: &str = r#"
fn main() {
    println!("Hello, limp!");
}"#;
const NAME: &str = "limp";
const CRATE_INFO_FILE: &str = "dependencies.db";
const SNIPPETS_DIR: &str = "snippets";

// bool - is windows
// pub struct FileManager {
//     is_windows: bool,
//     pub del: &'static str,
// }
// impl Default for FileManager {
//     fn default() -> Self {
//         let is_windows = std::env::consts::OS == "windows";
//         Self {
//             is_windows,
//             del: if is_windows { "\\" } else { "/" },
//         }
//     }
// }
//
// impl FileManager {
//     pub fn get_username(&self) -> String {
//         std::env::var("USER")
//             .unwrap_or(std::env::var("USERNAME").unwrap_or(String::from("unknown")))
//     }
//     pub fn config_dir(&self) -> String {
//         let username = self.get_username();
//         if self.is_windows {
//             format!("C:\\Users\\{}\\AppData\\Roaming\\{}", &username, NAME)
//         } else {
//             format!("/home/{}/.config/{}", &username, NAME)
//         }
//     }
//     pub fn config_file(&self) -> String {
//         let dir_path = self.config_dir();
//         format!("{}{}{}", &dir_path, self.del, CRATE_INFO_FILE)
//     }
//     pub fn storage_dir(&self) -> String {
//         let dir_path = self.config_dir();
//         format!("{}{}{}", &dir_path, self.del, SNIPPETS_DIR)
//     }
//
//     pub fn create_project(&self, name: &str) -> std::io::Result<()> {
//         // path is a relative path or an absolute path
//         let src = format!("{}{}src", name, self.del);
//         let main = format!("{}{}main.rs", &src, self.del);
//         let cargo = format!("{}{}Cargo.toml", name, self.del);
//
//         std::fs::create_dir_all(src)?;
//         let mut main = std::fs::File::create(&main)?;
//         main.write_all(MAIN_SNIP.as_bytes())?;
//         std::fs::File::create(&cargo)?;
//
//         if !std::process::Command::new("git")
//             .args(["init", name])
//             .spawn()?
//             .wait()?
//             .success()
//         {
//             eprintln!("ERROR: cannot create git repo in project {name}");
//             std::process::exit(1);
//         }
//         let gitignore = format!("{}{}.gitignore", name, self.del);
//         let mut file = self.copen(&gitignore);
//         file.write_all(b"/target")?;
//
//         Ok(())
//     }
//     pub fn copen(&self, path: &str) -> std::fs::File {
//         let path = std::path::Path::new(path);
//         if path.exists() {
//             std::fs::File::options()
//                 .read(true)
//                 .write(true)
//                 t
//                 .open(path)
//                 .unwrap()
//         } else {
//             std::fs::File::create(path).unwrap_or_else(|_| {
//                 if let Some(parent) = path.parent() {
//                     std::fs::create_dir_all(parent).unwrap();
//                     std::fs::File::create(path).unwrap()
//                 } else {
//                     panic!("Not reachable")
//                 }
//             })
//         }
//     }
// }

pub fn username() -> String {
    std::env::var("USER").unwrap_or(std::env::var("USERNAME").unwrap_or("unknown".to_string()))
}

pub fn storage_path() -> PathBuf {
    let uname = username();

    match std::env::consts::OS {
        "windows" => PathBuf::from(format!("C:\\Users\\{}\\AppData\\Roaming\\limp", &uname)),
        _ => PathBuf::from(format!("/home/{}/.config/limp/", &uname)),
    }
}

pub fn config_path() -> PathBuf {
    storage_path().join("dependencies.json")
}

pub fn snippets_dir() -> PathBuf {
    storage_path().join("snippets")
}

pub fn find_toml() -> Option<PathBuf> {
    if let Ok(mut path) = std::env::current_dir() {
        let pre_toml = path.join("Cargo.toml");
        if pre_toml.exists() {
            return Some(pre_toml);
        }
        while path.pop() {
            let pre_toml = path.join("Cargo.toml");
            if pre_toml.exists() {
                return Some(pre_toml);
            }
        }
        return None;
    }
    None
}

pub fn open<P: AsRef<Path>>(path: P) -> Result<File, LimpError> {
    let path = path.as_ref();
    fs::create_dir_all(path.parent().unwrap_or(Path::new("./")))?;
    let file = File::options()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;
    Ok(file)
}

pub fn create_project(name: &str, deps: Option<&[String]>) -> Result<(), LimpError> {
    let project = PathBuf::from(format!("./{}", name));
    if project.exists() && project.read_dir()?.count() > 0 {
        return Err(LimpError::CrateExistsNotEmpty(name.to_string()));
    }

    let mut toml = open(project.join("Cargo.toml"))?;
    writeln!(toml, "[package]")?;
    writeln!(toml, "name = \"{}\"", name)?;
    writeln!(toml, "version = \"0.1.0\"")?;
    writeln!(toml, "edition = \"2021\"")?;
    writeln!(toml)?;
    writeln!(toml, "[dependencies]")?;
    if let Some(deps) = deps {
        for dep in deps.iter() {
            writeln!(toml, "{}", dep)?
        }
    }

    let mut main = open(project.join("src").join("main.rs"))?;
    main.write_all(MAIN_SNIP.as_bytes())?;

    if !std::process::Command::new("git")
        .args(["init", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?
        .success()
    {
        return Err(LimpError::GitError(name.to_string()));
    }
    let mut gitignore = open(project.join(".gitignore"))?;
    gitignore.write_all(b"/target")?;

    Ok(())
}
