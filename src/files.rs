use std::io::Write;

const MAIN_SNIP: &str = r#"
fn main() {
    println!("Hello, limp!");
}"#;
const NAME: &str = "limp";
const CRATE_INFO_FILE: &str = "dependencies.json";
const SNIPPETS_DIR: &str = "snippets";

pub struct FileManager;

impl FileManager {
    pub fn get_username() -> Option<String> {
        let output = std::process::Command::new("whoami")
            .output()
            .expect("Failed to execute whoami command");

        if output.status.success() {
            // Convert the output bytes to a string and remove the newline character
            let mut username = String::from_utf8(output.stdout)
                .expect("Failed to convert output to string")
                .trim()
                .to_string();
            if std::env::consts::OS == "windows" {
                username = username.split_once("\\").unwrap().1.to_string();
            }

            Some(username)
        } else {
            None
        }
    }
    pub fn config_path() -> Option<String> {
        match Self::get_username() {
            Some(uname) => {
                if std::env::consts::OS == "windows" {
                    return Some(format!(
                        "C:\\Users\\{}\\AppData\\Roaming\\{}\\{}",
                        uname, NAME, CRATE_INFO_FILE
                    ));
                } else {
                    return Some(format!(
                        "/home/{}/.config/{}/{}",
                        uname, NAME, CRATE_INFO_FILE
                    ));
                }
            }
            None => {
                if std::env::consts::OS == "windows" {
                    if let Ok(uname) = std::env::var("USERNAME") {
                        return Some(format!(
                            "C:\\Users\\{}\\AppData\\Roaming\\{}\\{}",
                            uname, NAME, CRATE_INFO_FILE
                        ));
                    }
                } else if let Ok(uname) = std::env::var("USER") {
                    return Some(format!(
                        "/home/{}/.config/{}/{}",
                        uname, NAME, CRATE_INFO_FILE
                    ));
                }
            }
        }
        None
    }

    pub fn create_project(name: &str) -> std::io::Result<()> {
        let spliter = match std::env::consts::OS {
            "windows" => "\\",
            _ => "/",
        };
        // path is a relative path or an absolute path
        let src = format!("{}{}src", name, spliter);
        let main = format!("{}{}main.rs", &src, spliter);
        let cargo = format!("{}{}Cargo.toml", name, spliter);

        std::fs::create_dir_all(src)?;
        let mut main = std::fs::File::create(&main)?;
        main.write_all(MAIN_SNIP.as_bytes())?;
        std::fs::File::create(&cargo)?;

        if !std::process::Command::new("git")
            .args(["init", name])
            .spawn()?
            .wait()?
            .success()
        {
            eprintln!("ERROR: cannot create git repo in project {name}");
            std::process::exit(1);
        }
        let gitignore = format!("{}{}.gitignore", name, spliter);
        let mut file = FileManager::copen(&gitignore);
        file.write_all(b"/target")?;

        Ok(())
    }
    pub fn copen(path: &str) -> std::fs::File {
        let path = std::path::Path::new(path);
        if path.exists() {
            std::fs::File::options()
                .read(true)
                .write(true)
                .open(path)
                .unwrap()
        } else {
            std::fs::File::create(path).unwrap_or_else(|_| {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                    std::fs::File::create(path).unwrap()
                } else {
                    panic!("Not reachable")
                }
            })
        }
    }
}
