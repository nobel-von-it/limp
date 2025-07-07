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

impl CommandHandler {
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
                                3 => v.to_string(),

                                2 => format!("{}.0", v),

                                1 => format!("{}.0.0", v),

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
