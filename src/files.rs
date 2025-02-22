use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use crate::{error::LimpError, parser::load_from_deps, storage::JsonDependency};

const MAIN_SNIP: &str = r#"fn main() {
    println!("Hello, limp!");
}"#;

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

pub fn add_to_snippets_dir(name: &str, content: &str) -> Result<String, LimpError> {
    let path = snippets_dir().join(format!("{name}.rs"));
    if path.exists() {
        return Err(LimpError::SnippetExists(name.to_string()));
    }
    let mut file = open(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(path.display().to_string())
}

pub fn remove_from_snippets_dir(name: &str) -> Result<(), LimpError> {
    let path = snippets_dir().join(format!("{name}.rs"));
    if !path.exists() {
        // This means the snippet doesn't provided by the user and nothing to remove
        return Ok(());
    }
    fs::remove_file(path)?;
    Ok(())
}

pub fn create_project(name: &str, deps: Option<&[JsonDependency]>) -> Result<(), LimpError> {
    let project = PathBuf::from(format!("./{}", name));
    if project.exists() && project.read_dir()?.count() > 0 {
        return Err(LimpError::CrateExistsNotEmpty(name.to_string()));
    }

    let mut main_snippet = MAIN_SNIP.to_string();
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
        main_snippet = load_from_deps(deps).unwrap_or(MAIN_SNIP.to_string());
    }

    let mut main = open(project.join("src").join("main.rs"))?;
    main.write_all(main_snippet.as_bytes())?;

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
