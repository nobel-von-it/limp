use clap::{Arg, ArgMatches, Command};

use crate::{error::LimpResult, models::Action, utils};

pub fn create_limp_command() -> Command {
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
                        .help("Optional features separated by comma"),
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

pub fn get_action(args: &ArgMatches) -> Option<Action> {
    Some(match args.subcommand() {
        Some(("init", subargs)) => {
            let name = subargs.get_one::<String>("name").unwrap().clone();
            let dependencies = subargs
                .get_many::<String>("dependencies")
                .map(|d| d.cloned().collect());
            Action::Init { name, dependencies }
        }
        Some(("new", subargs)) => {
            let name = subargs.get_one::<String>("name").unwrap().clone();
            let version = subargs
                .get_one::<String>("version")
                .map(|v| utils::parser::parse_dependency_version(v.to_string()));
            let features = subargs
                .get_one::<String>("features")
                .map(|f| f.split(",").map(|s| s.to_string()).collect::<Vec<String>>());
            let path_to_snippet = subargs
                .get_one::<String>("path_to_snippet")
                .map(|p| p.to_string());
            Action::NewDependency {
                name,
                version,
                features,
                path_to_snippet,
            }
        }
        Some(("del", subargs)) => {
            let name = subargs.get_one::<String>("name").unwrap().clone();
            Action::Delete { name }
        }
        Some(("add", subargs)) => {
            let name = subargs.get_one::<String>("name").unwrap().clone();
            Action::Add { name }
        }
        Some(("link", subargs)) => {
            let name = subargs.get_one::<String>("name").unwrap().clone();
            let path_to_snippet = subargs
                .get_one::<String>("path_to_snippet")
                .unwrap()
                .clone();
            Action::Link {
                name,
                path_to_snippet,
            }
        }
        Some(("unlink", subargs)) => {
            let name = subargs.get_one::<String>("name").unwrap().clone();
            Action::Unlink { name }
        }
        Some(("update", _)) => Action::Update,
        Some(("list", _)) => Action::List,
        _ => return None,
    })
}

pub fn execute_action(action: Action) -> LimpResult<()> {
    match action {
        Action::Init { name, dependencies } => utils::actions::init(name, dependencies),
        Action::NewDependency {
            name,
            version,
            features,
            path_to_snippet,
        } => actions::new(name, version, features, path_to_snippet),
        Action::Delete { name } => actions::delete(name),
        Action::Add { name } => actions::add(name),
        Action::Link {
            name,
            path_to_snippet,
        } => actions::link(name, path_to_snippet),
        Action::Unlink { name } => actions::unlink(name),
        Action::List => actions::list(),
        Action::Update => actions::update(),
    }
    Ok(())
}
