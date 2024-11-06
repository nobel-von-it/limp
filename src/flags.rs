use std::process::exit;

use crate::{actions::Action, eusage, to_version_string};

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

                    let mut version = None;
                    let mut path_to_snippet = None;
                    let mut features = None;
                    while let Some(arg) = args.next() {
                        match arg.as_str() {
                            "-v" | "--version" => {
                                if let Some(ver) = args.next() {
                                    version = Some(to_version_string(ver));
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
                "remove" | "del" | "delete" => {
                    if let Some(name) = args.next() {
                        config.action = Some(Action::Delete {
                            name: name.to_string(),
                        })
                    } else {
                        eusage();
                        exit(1);
                    }
                }
                "list" | "all" | "show-all" => config.action = Some(Action::List),
                "update" | "up" => config.action = Some(Action::Update),
                "v" | "version" => config.action = Some(Action::Version),
                "h" | "-h" | "help" | "--help" => config.action = Some(Action::Help),
                _ => {
                    eusage();
                    exit(1);
                }
            }
        }
        config
    }
    pub fn make_action(&self) {
        if let Some(act) = &self.action {
            match act {
                Action::Init { name, dependencies } => Action::init(name, dependencies),
                Action::NewDependency {
                    name,
                    version,
                    features,
                    path_to_snippet,
                } => Action::add_new(name, version, features, path_to_snippet),
                Action::Delete { name } => Action::delete(name),
                Action::List => Action::list(),
                Action::Update => Action::update(),
                Action::Version => Action::version(),
                Action::Help => eusage(),
            }
        }
    }
}
