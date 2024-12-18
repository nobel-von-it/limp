# Limp - Rust Project Management CLI

## Overview

Limp is a simple Command Line Interface (CLI) tool designed to streamline dependency management for Rust projects. It provides an easy way to initialize projects, add dependencies, and manage your project's configuration.


## Features


- **Effortless Project Initialization**: Quickly set up new projects with a single command.
- **Dependency Management**: Add and manage dependencies with ease, including version and feature handling.
- **Integration with Crates.io**: If you want, fetch the latest versions and features of dependencies from Crates.io.
- **Configuration Management**: Manage your project's dependencies and configuration easily.
- **Code Generation**: Generate code snippets based on dependencies and configuration. (*yet not supported*)

## Installation


You have two primary methods to install and use Limp:

### Option 1: Install via Cargo (Recommended)
```bash
cargo install limp
```
After installation, you can use Limp directly:
```bash
limp init my_project
limp new serde
# Other commands...
```

### Option 2: Clone and Run from Source
If you want to use the latest development version or contribute to the project:
```bash
git clone --depth=1 https://github.com/nobel-von-it/limp
cd limp
cargo run -- init my_project
cargo run -- new serde
# Use cargo run -- before each command when running from source
```

### Requirements
- Rust toolchain (rustc, cargo)
- Git (for source installation)
## Usage/Examples

### 1. Initialize a New Project
```bash
limp init <project-name> [-d <dependencies>]
```
- Creates a new Rust project
- Optional: Specify dependencies during initialization with `-d` flag
- Example: `limp init my_project -d serde tokio`

### 2. Add a New Dependency
```bash
limp new <dependency-name> [options]
```
Options:
- `-v, --version <version>`: Specify dependency version
- `-p, --path <path_to_snippet>`: Path to a code snippet (*yet not supported*)
- `-f, --features <feature1> <feature2>`: Enable specific features

Example: 
```bash
limp new serde -v 1.0.0 -f derive
```

### 3. Delete a Dependency
```bash
limp del <dependency-name>
```
Removes a dependency from your configuration

### 4. Add Dependency to Existing Project
```bash
limp add <dependency-name>
```
Adds a dependency directly to the current project's `Cargo.toml`

### 5. List Dependencies
```bash
limp list
```
Displays all configured dependencies

### 6. Update Dependencies
```bash
limp update
```
Updates all dependencies to their latest versions

