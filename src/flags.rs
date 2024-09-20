use std::process::exit;

use crate::{
    crates::CrateValidator,
    eusage,
    json::{self, JsonDependencies},
    toml::{Dependency, Project},
    usage,
};

pub enum Action {
    Init {
        name: String,
        dependencies: Option<Vec<String>>,
    },
    NewDependency {
        name: String,
        version: String,
        features: Option<Vec<String>>,
        path_to_snippet: Option<String>,
    },
    HelloWorld,
}

#[derive(Default)]
pub struct Config {
    pub action: Option<Action>,
}
impl Config {
    pub fn parse(args: &[String]) -> Self {
        let mut config = Config::default();
        let mut args = args.iter();

        if let Some(action) = args.next() {
            match action.as_str() {
                "init" => {
                    let name = args
                        .next()
                        .unwrap_or_else(|| {
                            eusage();
                            exit(1);
                        })
                        .to_string();
                    let dependencies = if let Some(deps_flag) = args.next() {
                        if deps_flag == "-d" || deps_flag == "--deps" {
                            let mut deps = vec![];
                            for dep in args.by_ref() {
                                deps.push(dep.to_string());
                            }
                            Some(deps)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    config.action = Some(Action::Init { name, dependencies })
                }
                "new" | "add" => {
                    let name = args
                        .next()
                        .unwrap_or_else(|| {
                            eusage();
                            exit(1);
                        })
                        .to_string();

                    let mut version = String::new();
                    let mut path_to_snippet = None;
                    let mut features = None;
                    while let Some(arg) = args.next() {
                        match arg.as_str() {
                            "-v" | "--version" => {
                                if let Some(ver) = args.next() {
                                    version = ver.to_string();
                                } else {
                                    eusage();
                                    exit(1);
                                }
                            }
                            "-p" | "--path" => {
                                if let Some(pts) = args.next() {
                                    path_to_snippet = Some(pts.to_string());
                                }
                            }
                            "-f" | "--features" => {
                                let mut fs = vec![];
                                for feature in args.by_ref() {
                                    fs.push(feature.to_string())
                                }
                                if !fs.is_empty() {
                                    features = Some(fs);
                                }
                            }
                            _ => {}
                        }
                    }
                    config.action = Some(Action::NewDependency {
                        name,
                        version,
                        features,
                        path_to_snippet,
                    })
                }
                "hello" | "helloworld" | "hw" => {
                    config.action = Some(Action::HelloWorld);
                }
                _ => {}
            }
        }
        config
    }
    pub fn make_action(&self) {
        if let Some(act) = &self.action {
            match act {
                Action::Init { name, dependencies } => {
                    println!("Initialize project with name {}", &name);
                    let jd = json::load();
                    let proj = Project {
                        name: name.to_string(),
                        dependencies: if let Some(dependencies) = dependencies {
                            dependencies
                                .iter()
                                .map(|d| json::get_dependency(&jd, d).expect("TODO"))
                                .collect()
                        } else {
                            vec![]
                        },
                        ..Default::default()
                    };
                    proj.write().map_err(|e| eprintln!("{e}")).unwrap()
                }
                Action::NewDependency {
                    name,
                    version,
                    features,
                    path_to_snippet,
                } => {
                    let mut jd = json::load();
                    if let Some(d) = json::add_new(
                        &mut jd,
                        name,
                        version,
                        features.clone(),
                        path_to_snippet.clone(),
                    ) {
                        println!("Add new dependency into json database");
                        println!("{d}");
                        json::save(&jd);
                    }
                }
                Action::HelloWorld => println!("Hello world from command"),
            }
        }
    }
}
