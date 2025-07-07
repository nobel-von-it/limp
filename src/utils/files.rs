use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use crate::{
    error::{LimpError, LimpResult},
    models::config::RunType,
};

const MAIN_SNIP: &str = r#"fn main() {
    println!("Hello, limp!");
}"#;

pub fn username() -> String {
    std::env::var("USER").unwrap_or(std::env::var("USERNAME").unwrap_or("unknown".to_string()))
}

pub fn storage_path(run_type: &RunType) -> String {
    match run_type {
        RunType::Test => test_storage_path(),
        RunType::Release => release_storage_path(),
        _ => dev_storage_path(),
    }
}

pub fn snippet_dir(run_type: &RunType) -> String {
    match run_type {
        RunType::Test => test_snippet_dir(),
        RunType::Release => release_snippet_dir(),
        _ => dev_snippet_dir(),
    }
}

pub fn release_storage_path() -> String {
    match std::env::consts::OS {
        "windows" => release_storage_path_win(),
        _ => release_storage_path_unix(),
    }
}

pub fn release_snippet_dir() -> String {
    match std::env::consts::OS {
        "windows" => release_snippet_dir_win(),
        _ => release_snippet_dir_unix(),
    }
}

pub fn release_storage_path_unix() -> String {
    let uname = username();
    format!("/home/{}/.config/limp/dependencies.json", &uname)
}

pub fn release_snippet_dir_unix() -> String {
    let uname = username();
    format!("/home/{}/.config/limp/snippets", &uname)
}

pub fn release_storage_path_win() -> String {
    let uname = username();
    format!(
        "C:\\Users\\{}\\AppData\\Roaming\\limp\\dependencies.json",
        &uname
    )
}

pub fn release_snippet_dir_win() -> String {
    let uname = username();
    format!("C:\\Users\\{}\\AppData\\Roaming\\limp\\snippets", &uname)
}

pub fn dev_storage_path() -> String {
    match std::env::consts::OS {
        "windows" => dev_storage_path_win(),
        _ => dev_storage_path_unix(),
    }
}

pub fn dev_snippet_dir() -> String {
    match std::env::consts::OS {
        "windows" => dev_snippet_dir_win(),
        _ => dev_snippet_dir_unix(),
    }
}

pub fn dev_storage_path_unix() -> String {
    let uname = username();
    format!("/home/{}/.local/share/limp/dependencies.json", &uname)
}

pub fn dev_snippet_dir_unix() -> String {
    let uname = username();
    format!("/home/{}/.local/share/limp/snippets", &uname)
}

pub fn dev_storage_path_win() -> String {
    let uname = username();
    format!(
        "C:\\Users\\{}\\AppData\\Local\\limp\\dependencies.json",
        &uname
    )
}

pub fn dev_snippet_dir_win() -> String {
    let uname = username();
    format!("C:\\Users\\{}\\AppData\\Local\\limp\\snippets", &uname)
}

pub fn test_storage_path() -> String {
    dev_storage_path()
}

pub fn test_snippet_dir() -> String {
    dev_snippet_dir()
}

pub fn test_storage_path_unix() -> String {
    dev_storage_path_unix()
}

pub fn test_snippet_dir_unix() -> String {
    dev_snippet_dir_unix()
}

pub fn test_storage_path_win() -> String {
    dev_storage_path_win()
}

pub fn test_snippet_dir_win() -> String {
    dev_snippet_dir_win()
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

pub fn open<P: AsRef<Path>>(path: P) -> LimpResult<File> {
    let path = path.as_ref();

    fs::create_dir_all(path.parent().unwrap_or(Path::new("./")))?;

    let file = File::options()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;
    Ok(file)
}

pub fn add_to_snippets_dir(name: &str, content: &str) -> LimpResult<String> {
    let path = ().join(format!("{name}.rs"));
    if path.exists() {
        return Err(LimpError::SnippetExists(name.to_string()));
    }
    let mut file = open(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(path.display().to_string())
}

pub fn remove_from_snippets_dir(name: &str) -> Result<(), LimpError> {
    let path = snippet_dir().join(format!("{name}.rs"));
    if !path.exists() {
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
