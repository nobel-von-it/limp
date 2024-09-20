use core::panic;
use std::io::Write;

const MAIN_SNIP: &str = r#"
fn main() {
    println!("Hello, limp!");
}"#;

pub struct FileManager;

impl FileManager {
    pub fn create_project(name: &str) -> std::io::Result<()> {
        // path is a relative path or an absolute path
        let src = format!("{}/src", name);
        let main = format!("{}/main.rs", &src);
        let cargo = format!("{}/Cargo.toml", name);

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
