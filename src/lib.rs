pub mod actions;
pub mod crates;
pub mod error;
pub mod files;
pub mod parser;
pub mod storage;

pub enum RunType {
    Release,
    Dev,
    Test,
    Custom(String),
}
