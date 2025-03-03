//! # Command Handler Module
//!
//! This module provides functionality for handling CLI commands in the `limp` tool.
//! It parses command-line arguments, maps them to specific actions, and executes those actions.

use std::io::{Read, Write};

use clap::{Arg, ArgMatches, Command};

use crate::{
    error::LimpError,
    files::{
        add_to_snippets_dir, config_path, create_project, find_toml, open, remove_from_snippets_dir,
    },
    parser::SnippetEntity,
    storage::{JsonDependency, JsonStorage},
};

/// Represents the actions that can be performed by the CLI.
///
/// Each variant corresponds to a specific command (e.g., `init`, `new`, `delete`, etc.).
pub enum Action {
    /// Initialize a new project.
    Init {
        /// The name of the project.
        name: String,
        /// Optional dependencies to include in the project.
        dependencies: Option<Vec<String>>,
    },
    /// Add a new dependency.
    NewDependency {
        /// The name of the dependency.
        name: String,
        /// The version of the dependency (optional).
        version: Option<String>,
        /// Optional features to enable for the dependency.
        features: Option<Vec<String>>,
        /// Path to a snippet associated with the dependency (optional).
        path_to_snippet: Option<String>,
    },
    /// Delete a dependency.
    Delete {
        /// The name of the dependency to delete.
        name: String,
    },
    /// Add a dependency to an existing project.
    Add {
        /// The name of the dependency to add.
        name: String,
    },
    /// Link a dependency to a snippet.
    Link {
        /// The name of the dependency.
        name: String,
        /// The path to the snippet to link.
        path_to_snippet: String,
    },
    /// Unlink a dependency from a snippet.
    Unlink {
        /// The name of the dependency to unlink.
        name: String,
    },
    /// Update all dependencies.
    Update,
    /// List all dependencies.
    List,
}
/// Handles CLI commands and executes corresponding actions.
///
/// This struct is responsible for parsing CLI arguments and mapping them to specific actions
/// (e.g., initializing a project, adding a dependency, etc.).
///
/// # Fields
/// - `action`: An optional `Action` enum representing the command to execute.
#[derive(Default)]
pub struct CommandHandler {
    pub action: Option<Action>,
}
impl CommandHandler {
    /// Builds the CLI command structure using `clap`.
    ///
    /// This function defines the CLI commands, arguments, and help messages.
    ///
    /// # Returns
    /// A `clap::Command` object representing the CLI structure.
    pub fn build() -> Command {
        Command::new("limp")
            .about("Limp is a simple CLI tool for managing your rust projects.")
            .version("v0.2.1")
            .subcommand_required(true)
            .subcommand(
                Command::new("init")
                    .about("Initialize a new project")
                    .arg(Arg::new("name").required(true))
                    .arg(
                        Arg::new("dependencies")
                            .required(false)
                            .short('d')
                            .long("dependencies")
                            .num_args(0..)
                            .help("Optional dependencies"),
                    ),
            )
            .subcommand(
                Command::new("new")
                    .about("Add a new dependency")
                    .arg(Arg::new("name").required(true))
                    .arg(
                        Arg::new("version")
                            .required(false)
                            .short('v')
                            .long("version")
                            .help("Specify version"),
                    )
                    .arg(
                        Arg::new("path_to_snippet")
                            .required(false)
                            .short('p')
                            .long("path")
                            .help("Path to snippet"),
                    )
                    .arg(
                        Arg::new("features")
                            .required(false)
                            .short('f')
                            .long("features")
                            .num_args(0..)
                            .help("Optional features"),
                    ),
            )
            .subcommand(
                Command::new("del")
                    .about("Delete dependency")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(
                Command::new("add")
                    .about("Add dependency to existing project")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(
                Command::new("link")
                    .about("Link dependency to snippet")
                    .arg(Arg::new("name").required(true))
                    .arg(Arg::new("path_to_snippet").required(true)),
            )
            .subcommand(
                Command::new("unlink")
                    .about("Unlink dependency from snippet")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(Command::new("list").about("List dependencies"))
            .subcommand(Command::new("update").about("Update dependencies"))
            .subcommand(Command::new("version").about("Print version"))
    }
    /// Parses CLI arguments and maps them to an `Action`.
    ///
    /// This function takes `ArgMatches` from `clap` and maps the subcommand to a specific `Action`.
    ///
    /// # Arguments
    /// * `args` - The `ArgMatches` object containing parsed CLI arguments.
    ///
    /// # Returns
    /// A `CommandHandler` instance with the `action` field set based on the parsed arguments.
    pub fn parse(args: &ArgMatches) -> Self {
        Self {
            action: match args.subcommand() {
                Some((subname, subargs)) => match subname {
                    "init" => Some(Action::Init {
                        name: subargs.get_one::<String>("name").unwrap().clone(),
                        dependencies: subargs
                            .get_many::<String>("dependencies")
                            .map(|d| d.cloned().collect()),
                    }),
                    "new" => Some(Action::NewDependency {
                        name: subargs.get_one::<String>("name").unwrap().clone(),
                        version: subargs.get_one::<String>("version").map(|v| {
                            match v
                                .split(".")
                                .map(|s| s.parse::<u16>().unwrap_or_default())
                                .collect::<Vec<u16>>()
                                .len()
                            {
                                // 1.1.1 -> nothing change
                                3 => v.to_string(),
                                // 0.25 -> 0.25.0
                                2 => format!("{}.0", v),
                                // 1 -> 1.0.0
                                1 => format!("{}.0.0", v),
                                // no way
                                _ => unreachable!(),
                            }
                        }),
                        features: subargs
                            .get_many::<String>("features")
                            .map(|f| f.cloned().collect()),
                        path_to_snippet: subargs.get_one::<String>("path_to_snippet").cloned(),
                    }),
                    "del" => Some(Action::Delete {
                        name: subargs.get_one::<String>("name").unwrap().clone(),
                    }),
                    "add" => Some(Action::Add {
                        name: subargs.get_one::<String>("name").unwrap().clone(),
                    }),
                    "link" => Some(Action::Link {
                        name: subargs.get_one::<String>("name").unwrap().clone(),
                        path_to_snippet: subargs
                            .get_one::<String>("path_to_snippet")
                            .unwrap()
                            .clone(),
                    }),
                    "unlink" => Some(Action::Unlink {
                        name: subargs.get_one::<String>("name").unwrap().clone(),
                    }),
                    "list" => Some(Action::List),
                    "update" => Some(Action::Update),
                    _ => None,
                },
                None => None,
            },
        }
    }
    /// Executes the action specified in the `CommandHandler`.
    ///
    /// This function performs the action (e.g., initializing a project, adding a dependency, etc.)
    /// based on the `action` field.
    ///
    /// # Returns
    /// - `Ok(())` if the action is executed successfully.
    /// - `Err(LimpError)` if an error occurs during execution.
    pub fn make_action(&self) -> Result<(), LimpError> {
        if let Some(act) = &self.action {
            match act {
                Action::Init { name, dependencies } => {
                    let js = JsonStorage::load(config_path())?;
                    let mut odeps = None;
                    if let Some(deps) = dependencies {
                        let mut result_deps = vec![];
                        for d in deps.iter() {
                            result_deps.push(match js.get(d) {
                                Some(d) => d.clone(),
                                None => JsonDependency::new(d)?,
                            });
                        }
                        if !result_deps.is_empty() {
                            odeps = Some(result_deps);
                        }
                    }
                    println!("Adding dependencies: {:?} to {}", odeps, name);
                    create_project(name, odeps.as_deref())?;
                    println!("Done");
                }
                Action::NewDependency {
                    name,
                    version,
                    features,
                    path_to_snippet,
                } => {
                    let mut js = JsonStorage::load(config_path())?;

                    let jd = JsonDependency::new_full(
                        name,
                        version.as_deref(),
                        features.as_deref(),
                        path_to_snippet.as_deref(),
                    )?;
                    js.add(jd);

                    js.save(config_path())?;
                    println!("Successfully added {}", name);
                }
                Action::Delete { name } => {
                    let mut js = JsonStorage::load(config_path())?;

                    js.remove(name);
                    remove_from_snippets_dir(name)?;

                    js.save(config_path())?;
                    println!("Successfully deleted {}", name);
                }
                Action::Add { name } => {
                    if let Some(path) = find_toml() {
                        let mut file = open(path)?;
                        let js = JsonStorage::load(config_path())?;

                        let mut content = String::new();
                        file.read_to_string(&mut content)?;

                        let deps = if let Some(existing_deps) = js.get(name) {
                            existing_deps.to_string()
                        } else {
                            JsonDependency::new(name)?.to_string()
                        };
                        if content.contains("[dependencies]") {
                            writeln!(file, "{}", deps)?
                        } else {
                            writeln!(file, "\n[dependencies]")?;
                            writeln!(file, "{}", deps)?
                        }
                    } else {
                        return Err(LimpError::CargoTomlNotFound(format!(
                            "dep: {}\npath: {}",
                            name,
                            std::env::current_dir().unwrap().display()
                        )));
                    }
                }
                Action::Link {
                    name,
                    path_to_snippet,
                } => {
                    let mut js = JsonStorage::load(config_path())?;

                    let p = SnippetEntity::from_file(path_to_snippet)?;
                    let path_to_snippet = add_to_snippets_dir(name, p.to_string().as_str())?;

                    js.dependencies
                        .get_mut(name)
                        .ok_or(LimpError::DependencyNotFound(name.to_string()))?
                        .path_to_snippet = Some(path_to_snippet.clone());

                    js.save(config_path())?;

                    println!("Successfully linked {} to {}", name, path_to_snippet);
                }
                Action::Unlink { name } => {
                    let mut js = JsonStorage::load(config_path())?;
                    js.dependencies
                        .get_mut(name)
                        .ok_or(LimpError::DependencyNotFound(name.to_string()))?
                        .path_to_snippet = None;

                    remove_from_snippets_dir(name)?;
                    js.save(config_path())?;

                    println!("Successfully unlinked {}", name);
                }
                Action::List => {
                    let js = JsonStorage::load(config_path())?;
                    js.dependencies.iter().enumerate().for_each(|(i, (_, d))| {
                        println!("{} id:", i + 1);
                        println!("  - {}", d.name);
                        println!("  - {}", d.version);
                        if let Some(f) = &d.features {
                            println!("  - {}", f.join(", "));
                        }
                        if let Some(p) = &d.path_to_snippet {
                            println!("  - {}", p);
                        }
                    });
                }
                Action::Update => {
                    let mut js = JsonStorage::load(config_path())?;
                    js.dependencies
                        .iter_mut()
                        .map(|(_, d)| d)
                        .try_for_each(|d| d.update())?;
                    js.save(config_path())?;
                    println!("Successfully updated all dependencies");
                }
            }
        }
        Ok(())
    }
}
