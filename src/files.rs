use std::io::Write;

const MAIN_SNIP: &str = r#"
fn main() {
    println!("Hello, limp!");
}"#;
const NAME: &str = "limp";
const CRATE_INFO_FILE: &str = "dependencies.json";
const SNIPPETS_DIR: &str = "snippets";

// bool - is windows
pub struct FileManager {
    is_windows: bool,
    pub del: &'static str,
}
impl Default for FileManager {
    fn default() -> Self {
        let is_windows = std::env::consts::OS == "windows";
        Self {
            is_windows,
            del: if is_windows { "\\" } else { "/" },
        }
    }
}

impl FileManager {
    pub fn get_username(&self) -> String {
        let output = std::process::Command::new("whoami")
            .output()
            .expect("Failed to execute whoami command");

        if output.status.success() {
            // Convert the output bytes to a string and remove the newline character
            let username = String::from_utf8(output.stdout)
                .expect("Failed to convert output to string")
                .trim()
                .to_string();
            if self.is_windows {
                return username.split_once("\\").unwrap().1.to_string();
            }
            username
        } else {
            if let Ok(username) = std::env::var(if self.is_windows { "USERNAME" } else { "USER" }) {
                return username;
            }
            panic!("Get username error!")
        }
    }
    pub fn config_dir(&self) -> String {
        let username = self.get_username();
        if self.is_windows {
            format!("C:\\Users\\{}\\AppData\\Roaming\\{}", &username, NAME)
        } else {
            format!("/home/{}/.config/{}", &username, NAME)
        }
    }
    pub fn config_file(&self) -> String {
        let dir_path = self.config_dir();
        format!("{}{}{}", &dir_path, self.del, CRATE_INFO_FILE)
    }
    pub fn storage_dir(&self) -> String {
        let dir_path = self.config_dir();
        format!("{}{}{}", &dir_path, self.del, SNIPPETS_DIR)
    }

    pub fn create_project(&self, name: &str) -> std::io::Result<()> {
        // path is a relative path or an absolute path
        let src = format!("{}{}src", name, self.del);
        let main = format!("{}{}main.rs", &src, self.del);
        let cargo = format!("{}{}Cargo.toml", name, self.del);

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
        let gitignore = format!("{}{}.gitignore", name, self.del);
        let mut file = self.copen(&gitignore);
        file.write_all(b"/target")?;

        Ok(())
    }
    pub fn copen(&self, path: &str) -> std::fs::File {
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
