pub mod crates;
pub mod error;
pub mod files;
pub mod flags;
pub mod json;
pub mod toml;
pub mod worker;

pub fn usage() -> String {
    format!("Usage: limp action <options> <args>")
}

pub fn eusage() {
    eprintln!("{}", usage());
}
