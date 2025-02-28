//! # File and Project Utilities Module
//!
//! This module provides utility functions for file and project management in the `limp` tool.
//! It includes functionality for creating projects, managing snippets, and interacting with the file system.

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use crate::{
    error::{LimpError, Result},
    parser::load_from_deps,
    storage::JsonDependency,
};

/// Default main snippet for new projects.
///
/// This is the default `main.rs` content used when creating a new project.
/// It simply prints a greeting message.
/// But in the future, it can be extended to include more functionality.
const MAIN_SNIP: &str = r#"fn main() {
    println!("Hello, limp!");
}"#;

/// Retrieves the current username.
///
/// This function retrieves the username of the current user from environment variables.
/// It checks `USER` (Unix-like systems) and `USERNAME` (Windows) and defaults to "unknown" if neither is found.
/// May be We could use `whoami` command to get the username or just return error if not found
///
/// # Returns
/// The username as a `String`.
pub fn username() -> String {
    std::env::var("USER").unwrap_or(std::env::var("USERNAME").unwrap_or("unknown".to_string()))
}

/// Returns the storage path for `limp` configuration and data.
///
/// This function determines the appropriate storage path based on the operating system:
/// - On Windows: `C:\Users\<username>\AppData\Roaming\limp`
/// - On Unix-like systems: `/home/<username>/.config/limp/`
///
/// # Returns
/// The storage path as a `PathBuf`.
pub fn storage_path() -> PathBuf {
    let uname = username();

    match std::env::consts::OS {
        // On Windows PathBuf doesn't work and return IO premission error
        "windows" => PathBuf::from(format!("C:\\Users\\{}\\AppData\\Roaming\\limp", &uname)),
        _ => PathBuf::from(format!("/home/{}/.config/limp/", &uname)),
    }
}

/// Returns the path to the `dependencies.json` configuration file.
///
/// This file stores dependency information for `limp`.
///
/// # Returns
/// The path to `dependencies.json` as a `PathBuf`.
pub fn config_path() -> PathBuf {
    storage_path().join("dependencies.json")
}

/// Returns the path to the snippets directory.
///
/// This directory stores code snippets managed by `limp`.
///
/// # Returns
/// The path to the snippets directory as a `PathBuf`.
pub fn snippets_dir() -> PathBuf {
    storage_path().join("snippets")
}

/// Locates the `Cargo.toml` file in the current or parent directories.
///
/// This function searches for `Cargo.toml` starting from the current directory
/// and traversing up the directory hierarchy.
///
/// # Returns
/// - `Some(PathBuf)` if `Cargo.toml` is found.
/// - `None` if `Cargo.toml` is not found.
pub fn find_toml() -> Option<PathBuf> {
    if let Ok(mut path) = std::env::current_dir() {
        // Check if `Cargo.toml` exists in the current directory because
        // it's more likely to be in the current directory and
        // while run `path.pop()` before checking `Cargo.toml`
        let pre_toml = path.join("Cargo.toml");
        if pre_toml.exists() {
            return Some(pre_toml);
        }
        // This is to traverse up the directory hierarchy
        // until `Cargo.toml` is found and it's not recursive (more convenient)
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

/// Opens a file, creating it and its parent directories if they don't exist.
///
/// This function ensures that the file and its parent directories exist before opening the file.
///
/// # Arguments
/// * `path` - The path to the file.
///
/// # Returns
/// - `Ok(File)` if the file is successfully opened.
/// - `Err(LimpError)` if an error occurs.
pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
    let path = path.as_ref();
    // Create parent directories if they don't exist
    fs::create_dir_all(path.parent().unwrap_or(Path::new("./")))?;
    // The file is opened in append mode to ensure that the file is created if it doesn't exist
    let file = File::options()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;
    Ok(file)
}

/// Adds a snippet to the snippets directory.
///
/// This function creates a new snippet file with the provided content.
///
/// # Arguments
/// * `name` - The name of the snippet.
/// * `content` - The content of the snippet.
///
/// # Returns
/// - `Ok(String)` with the path to the snippet file if successful.
/// - `Err(LimpError)` if the snippet already exists or an error occurs.
pub fn add_to_snippets_dir(name: &str, content: &str) -> Result<String> {
    let path = snippets_dir().join(format!("{name}.rs"));
    if path.exists() {
        return Err(LimpError::SnippetExists(name.to_string()));
    }
    let mut file = open(&path)?;
    file.write_all(content.as_bytes())?;
    // Use `display` to get the path as a readable string
    Ok(path.display().to_string())
}

/// Removes a snippet from the snippets directory.
///
/// This function deletes the snippet file if it exists.
///
/// # Arguments
/// * `name` - The name of the snippet.
///
/// # Returns
/// - `Ok(())` if the snippet is removed or does not exist.
/// - `Err(LimpError)` if an error occurs.
pub fn remove_from_snippets_dir(name: &str) -> Result<()> {
    let path = snippets_dir().join(format!("{name}.rs"));
    if !path.exists() {
        // This means the snippet doesn't provided by the user and nothing to remove
        return Ok(());
    }
    fs::remove_file(path)?;
    Ok(())
}

/// Creates a new Rust project.
///
/// This function initializes a new Rust project with a `Cargo.toml` file, a `main.rs` file,
/// and optionally includes dependencies.
///
/// # Arguments
/// * `name` - The name of the project.
/// * `deps` - Optional dependencies to include in the project.
///
/// # Returns
/// - `Ok(())` if the project is created successfully.
/// - `Err(LimpError)` if the project already exists or an error occurs.
pub fn create_project(name: &str, deps: Option<&[JsonDependency]>) -> Result<()> {
    let project = PathBuf::from(format!("./{}", name));
    // Check if the project already exists or is not empty
    if project.exists() && project.read_dir()?.count() > 0 {
        return Err(LimpError::CrateExistsNotEmpty(name.to_string()));
    }

    let mut main_snippet = MAIN_SNIP.to_string();
    let mut toml = open(project.join("Cargo.toml"))?;
    // Write the `Cargo.toml` file base config
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

    // Initialize git by running `git init` (this will create a .git folder)
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
    // Based on `cargo new` command, add `target` to .gitignore
    let mut gitignore = open(project.join(".gitignore"))?;
    gitignore.write_all(b"/target")?;

    Ok(())
}
