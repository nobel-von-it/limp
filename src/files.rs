use core::panic;

use crate::error::IError;

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
    pub fn create_project(path: &str) -> Result<(), IError> {
        let path = std::path::Path::new(path);
        if path.exists() {
            return Err(IError::CreateFilesError);
        }
        std::fs::File::create(path).unwrap_or_else(|_| {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::File::create(path).unwrap()
        });
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
