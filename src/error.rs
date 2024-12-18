#[derive(thiserror::Error, Debug)]
pub enum LimpError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Crate already exists: {0}")]
    CrateExists(String),
    #[error("Crate already exists and is not empty: {0}")]
    CrateExistsNotEmpty(String),
    #[error("Parser error: {0}")]
    ParserError(#[from] serde_json::Error),
    #[error("Git creation error: {0}")]
    GitError(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] Box<ureq::Error>),
    #[error("Crate not found: {0}")]
    CrateNotFound(String),
    #[error("Version not found: {0}")]
    VersionNotFound(String),
    #[error("Snippet not found: {0}")]
    SnippetNotFound(String),
    #[error("Incompatible features: {0}")]
    IncompatibleFeatures(String),
    #[error("Cannot add dependency: {0}")]
    CargoTomlNotFound(String),
}
