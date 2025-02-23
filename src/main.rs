//! # Limp CLI - Main Entry Point
//!
//! This binary is the entry point for the `limp` command-line tool.
//! It parses command-line arguments, executes the corresponding actions,
//! and handles errors gracefully.
//!
//! ## Usage
//! Run the `limp` command with a subcommand to perform specific actions:
//! ```bash
//! limp init my_project          # Initialize a new project
//! limp new serde --version 1.0  # Add a new dependency
//! limp list                     # List all dependencies
//! ```
//!
//! For a full list of commands and options, run:
//! ```bash
//! limp --help
//! ```
//!
//! ## Error Handling
//! If an error occurs during execution, the program will print an error message
//! and exit with a non-zero status code.

use limp::actions::CommandHandler;

fn main() {
    let matches = CommandHandler::build().get_matches();
    let ch = CommandHandler::parse(&matches);
    if let Err(e) = ch.make_action() {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}
