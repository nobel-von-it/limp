pub mod actions;
pub mod crates;
pub mod error;
pub mod files;
pub mod flags;
pub mod json;
pub mod parser;
pub mod sqlite;
pub mod toml;

pub fn eusage() {
    eprintln!(
        "Usage: limp <command> [<args>]

Commands:
  init <name> [-d <dependencies>...]
    Initialize a new project with the given name and optional dependencies

  new <name> [-v <version>] [-f <features>...] [-p <path-to-snippet>]
    Add a new dependency to the project

  del <name>
    Delete dependency from database

  list | all | show-all
    List all dependencies

  help
    Show usage
"
    );
}

pub fn to_version_string(num: &str) -> String {
    match num
        .split(".")
        .map(|s| s.parse::<u16>().unwrap_or_default())
        .collect::<Vec<u16>>()
        .len()
    {
        // 1.1.1 -> nothing change
        3 => num.to_string(),
        // 0.25 -> 0.25.0
        2 => format!("{}.0", num),
        // 1 -> 1.0.0
        1 => format!("{}.0.0", num),
        // this's impossible
        _ => unreachable!(),
    }
}
