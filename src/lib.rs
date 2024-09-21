pub mod actions;
pub mod crates;
pub mod error;
pub mod files;
pub mod flags;
pub mod json;
pub mod parser;
pub mod toml;

pub fn eusage() {
    eprintln!(
        "Usage: limp <command> [<args>]

Commands:
  init <name> [-d <dependencies>...]
    Initialize a new project with the given name and optional dependencies.

  new <name> [-v <version>] [-f <features>...] [-p <path-to-snippet>]
    Add a new dependency to the project.

  list | all | show-all
    List all dependencies.

  help
    Show usage
"
    );
}
