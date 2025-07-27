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

    #[error("Snippet exists: {0}")]
    SnippetExists(String),

    #[error("Incompatible features: {0}")]
    IncompatibleFeatures(String),

    #[error("Cannot add dependency: {0}")]
    CargoTomlNotFound(String),

    #[error("Empty file: {0}")]
    EmptyFile(String),

    #[error("Not supported: {0}")]
    NotSupported(String),

    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectTypeError {
    #[error("Provided bin and lib: cannot initialize project")]
    BothSpecified,
    #[error("Provided nothing: cannot initialize project")]
    NeitherSpecified,
    #[error("Provided invalid project type: {0}")]
    InvalidType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum CompilerEditionError {
    #[error("Provided imvalid compiler edition: {0}")]
    InvalidEdition(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyTypeError {
    #[error("Provided incompatible denendency types")]
    IncompatibleCargoTypes,
    #[error("Provided invalid stringify dependency type: {0}")]
    InvalidType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyFeaturesError {
    #[error("Provided incompatible denendency features")]
    IncompatibleFeatures,
    #[error("Provided invalid features {0}")]
    InvalidFeature(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyTargetTypeError {
    #[error("Provided incompatible denendency target: {0}")]
    InvalidType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyNameError {
    #[error("Provided invalid dependency name: {0}")]
    InvalidName(String),
    #[error("Provided invalid dependency version: {0}")]
    InvalidVersion(#[from] DependencyVersionError),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyVersionError {
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Invalid parts: {0}")]
    InvalidParts(u8),
}
