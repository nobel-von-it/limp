use core::panic;
use std::io::Write;

// pub fn create(name: &str, is_lib: bool, add_git: bool) -> Result<Paths, Error> {
//     use super::{new_dir, new_file, read_write_file};
//     // create directory for project
//     if std::path::Path::new(name).exists() {
//         return Err(Error::CreateProjectError(format!(
//             "directory {name} already exists"
//         )));
//     }
//     let src = format!("{name}/src");
//     new_dir(&src)?;
//
//     // create main or lib file
//     let main = if is_lib {
//         format!("{}/lib.rs", &src)
//     } else {
//         format!("{}/main.rs", &src)
//     };
//     new_file(&main)?;
//
//     // create Cargo.toml file
//     let cargo = format!("{name}/Cargo.toml");
//     new_file(&cargo)?;
//
//     // create .git
//     if add_git {
//         let mut com = std::process::Command::new("git");
//         com.args(["init", name]);
//         // create .gitignore
//         if let Ok(_) = Process::command(&mut com) {
//             let gitignore = format!("{name}/.gitignore");
//             new_file(&gitignore)?;
//             let mut file = read_write_file(&gitignore)?;
//             if let Err(e) = file.write(b"/target") {
//                 return Err(Error::IO(e));
//             }
//         }
//     }
//     Ok(Paths {
//         name: name.to_string(),
//         toml: cargo,
//         main,
//     })
// }

pub struct FileManager;

impl FileManager {
    pub fn create_project(name: &str) -> std::io::Result<()> {
        // path is a relative path or an absolute path
        let src = format!("{}/src", name);
        let main = format!("{}/main.rs", &src);
        let cargo = format!("{}/Cargo.toml", name);

        std::fs::create_dir_all(src)?;
        std::fs::File::create(&main)?;
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
        let gitignore = format!("{}/.gitignore", name);
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
