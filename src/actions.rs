use std::io::{Read, Write};

use clap::{Arg, ArgMatches, Command};

use crate::{
    error::LimpError,
    files::{config_path, create_project, find_toml, open, remove_from_snippets_dir},
    storage::{JsonDependency, JsonStorage},
};
pub enum Action {
    Init {
        name: String,
        dependencies: Option<Vec<String>>,
    },
    NewDependency {
        name: String,
        version: Option<String>,
        features: Option<Vec<String>>,
        path_to_snippet: Option<String>,
    },
    Delete {
        name: String,
    },
    Add {
        name: String,
    },
    Update,
    List,
}

#[derive(Default)]
pub struct CommandHandler {
    pub action: Option<Action>,
}
impl CommandHandler {
    pub fn build() -> Command {
        Command::new("limp")
            .about("Limp is a simple CLI tool for managing your rust projects.")
            .version("v0.2.0")
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
            .subcommand(Command::new("list").about("List dependencies"))
            .subcommand(Command::new("update").about("Update dependencies"))
            .subcommand(Command::new("version").about("Print version"))
    }
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
                    "list" => Some(Action::List),
                    "update" => Some(Action::Update),
                    _ => None,
                },

                None => None,
            },
        }
    }
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
                Action::List => {
                    let js = JsonStorage::load(config_path())?;
                    js.dependencies
                        .iter()
                        .enumerate()
                        .for_each(|(i, (_, d))| println!("{}: {}", i + 1, d));
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
