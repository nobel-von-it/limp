//! # Limp Error Module
//!
//! This module provides custom error types for the `limp` tool.
//! It defines the `LimpError` enum, which encapsulates all possible errors that can occur during the execution of `limp`.
//!

/// Alias for the `Result` type with the `LimpError` error type.
pub type Result<T> = std::result::Result<T, LimpError>;

/// Represents errors that can occur in the `limp` tool.
///
/// This enum encapsulates all possible errors that might arise during the execution of `limp`,
/// including I/O errors, parsing errors, HTTP errors, and custom errors specific to `limp`.
///
/// Each variant provides a descriptive error message to help diagnose issues.
#[derive(thiserror::Error, Debug)]
pub enum LimpError {
    /// An I/O error occurred.
    ///
    /// This error is returned when an operation involving file I/O fails,
    /// such as reading or writing to a file.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    /// The specified crate already exists.
    ///
    /// This error is returned when attempting to create a crate that already exists.
    #[error("Crate already exists: {0}")]
    CrateExists(String),

    /// The specified crate already exists and is not empty.
    #[error("Crate already exists and is not empty: {0}")]
    CrateExistsNotEmpty(String),

    /// A parsing error occurred (e.g., invalid JSON).
    #[error("Parser error: {0}")]
    ParserError(#[from] serde_json::Error),

    /// An error occurred while interacting with Git.
    #[error("Git creation error: {0}")]
    GitError(String),

    /// An HTTP error occurred.
    ///
    /// This error is returned when an HTTP request fails,
    /// such as when fetching crate metadata from crates.io.
    /// `Box<T>` is used to optimize memory usage in case of large errors.
    #[error("HTTP error: {0}")]
    HttpError(#[from] Box<ureq::Error>),

    /// The specified crate was not found.
    #[error("Crate not found: {0}")]
    CrateNotFound(String),

    /// The specified version was not found.
    #[error("Version not found: {0}")]
    VersionNotFound(String),

    /// The specified snippet was not found.
    #[error("Snippet not found: {0}")]
    SnippetNotFound(String),

    /// The specified snippet already exists.
    #[error("Snippet exists: {0}")]
    SnippetExists(String),

    /// Incompatible features were specified.
    #[error("Incompatible features: {0}")]
    IncompatibleFeatures(String),

    /// The `Cargo.toml` file was not found.
    ///
    /// This error is returned when attempting to add a dependency
    /// but the `Cargo.toml` file is missing.
    #[error("Cannot add dependency: {0}")]
    CargoTomlNotFound(String),

    /// The specified file is empty.
    #[error("Empty file: {0}")]
    EmptyFile(String),

    /// The requested operation is not supported.
    #[error("Not supported: {0}")]
    NotSupported(String),

    /// The specified dependency was not found.
    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),

    /// The specified dependency already exists.
    #[error("Dependency already exists: {0}")]
    DependencyAlreadyExists(String),
}
