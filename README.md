 # LIMP: Library for Initialize ManiPulation

LIMP is a powerful tool designed to streamline project initialization and dependency management. With LIMP, you can easily create new projects and manage dependencies, ensuring a smooth development experience.

## Features

- **Effortless Project Initialization:** Quickly set up new projects with a single command.
- **Dependency Management:** Add and manage dependencies with ease, including version and feature handling.
- **Integration with Crates.io:** Automatically fetch the latest versions and features of dependencies from Crates.io.

## Getting Started

### Prerequisites

- Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).

### Installation

To install LIMP, you can clone the repository and build it locally:

```sh
git clone https://github.com/yourusername/limp
cd limp
cargo build --release
```

Or with cargo:

```sh
cargo install limp
```

## Usage

LIMP provides three main commands:

###   Initialize a New Project:

```sh
limp init my-project -d serde tokio
```
    my-project: The name of the project.
    -d or --deps: A list of dependencies to include (use at the end).

###   Add a New Dependency:

```sh
limp new/add serde -v 1.0.123 -p path/to/snippet.rs -f derive serde_derive 
```
    serde: The name of the dependency.
    -v or --version: The version of the dependency (optional, by default use latest).
    -f or --features: A list of features to enable (optional, use at the end).
    -p or --path: The path to a code snippet related to the dependency (optional).

###   Help:

```sh
limp del/delete/remove serde 
```
    serde: The name of the dependency.

###   List added Dependencies:

```sh
limp list 
```

###   Help:

```sh
limp help/h/-h/--help
```

