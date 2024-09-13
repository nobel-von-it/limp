pub mod crates;
pub mod flags;
pub mod toml;

pub fn usage() -> String {
    format!("Usage: limp action <options> <args>")
}
