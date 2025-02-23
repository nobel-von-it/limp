//! # Limp - CLI Tool for Managing Rust Projects
//!
//! `limp` is a command-line tool designed to simplify the management of Rust projects.
//! It provides utilities for creating projects, managing dependencies, and organizing code snippets.
//!
//! ## Key Features
//! - **Project Initialization**: Easily create new Rust projects with predefined templates.
//! - **Dependency Management**: Add, remove, and update dependencies with ease.
//! - **Snippet Management**: Store and link reusable code snippets.
//! - **Integration with Cargo**: Seamlessly works with existing `Cargo.toml` files.
//!
//! ## Modules
//! - [`actions`]: Handles CLI commands and executes corresponding actions.
//! - [`crates`]: Provides functionality for interacting with crates.io.
//! - [`error`]: Defines custom error types for the `limp` tool.
//! - [`files`]: Utilities for file and project management.
//! - [`parser`]: Parses dependencies and snippets.
//! - [`storage`]: Manages persistent storage for dependencies and snippets.
//!
//! ## Quick Start
//! To get started with `limp`, install it using Cargo:
//! ```bash
//! cargo install limp
//! ```
//!
//! Then, use the CLI to manage your projects:
//! ```bash
//! limp init my_project
//! limp new serde --version 1.0
//! cd my_project
//! limp add serde
//! ```
//!
//! For more details, see the documentation for individual modules.

pub mod actions;
pub mod crates;
pub mod error;
pub mod files;
pub mod parser;
pub mod storage;
