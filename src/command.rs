use std::path::PathBuf;

use clap::{ArgMatches, Command};

use crate::{
    arg::ArgManager,
    dependency::{DependencyFeatures, DependencyName, DependencyType},
    project::{CompilerEdition, ProjectType},
};

pub trait ClapCommandParser: Sized {
    fn command(am: &ArgManager) -> Command;
    // TODO: rewrite to LimpRestlt<Self>
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let _ = args;
        None
    }
}

#[derive(Debug)]
pub enum LimpCommand {
    Init(InitCommand),
    New(NewCommand),
    Add(AddCommand),
    StorageAdd(StorageAddCommand),
}

impl LimpCommand {
    pub fn parse(args: &ArgMatches) -> Option<Self> {
        if let Some((subname, subargs)) = args.subcommand() {
            return match subname {
                "init" => Some(LimpCommand::Init(InitCommand::from_matches(subargs)?)),
                "new" => Some(LimpCommand::New(NewCommand::from_matches(subargs)?)),
                "add" => Some(LimpCommand::Add(AddCommand::from_matches(subargs)?)),
                "store" => {
                    if let Some((subsubname, subsubargs)) = subargs.subcommand() {
                        match subsubname {
                            "add" => Some(LimpCommand::StorageAdd(
                                StorageAddCommand::from_matches(subsubargs)?,
                            )),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct DefaultConfig {
    verbose: bool,
    quiet: bool,
}

impl DefaultConfig {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }
}

#[derive(Debug)]
pub struct StorageConfig {
    path_to_storage: PathBuf,
}

#[derive(Debug)]
pub struct InitCommand {
    project_type: ProjectType,
    edition: CompilerEdition,
    name: String,
    config: DefaultConfig,
}

impl ClapCommandParser for InitCommand {
    fn command(am: &ArgManager) -> Command {
        Command::new("init")
            .about("Initialize existing project")
            .arg(am.base("name", false, "Project name"))
            .arg(am.flag_short_bool(
                "bin",
                false,
                "Set project type to binary (default)",
                'b',
                false,
            ))
            .arg(am.flag_short_bool("lib", false, "Set project type to lib", 'l', false))
            .arg(am.flag_long("project-type", false, "Project type (bin, lib)"))
            .arg(am.flag_short_val(
                "edition",
                false,
                "Rust edition (2015, 2018, 2021, 2024)",
                'e',
                "2024",
            ))
            .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
            .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
    }
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        // String and &str -> ok
        // &String and &str -> bruh
        let name = args
            .get_one::<String>("name")
            .unwrap_or(
                &std::env::current_dir()
                    .unwrap()
                    .file_name()?
                    .to_str()?
                    .to_string(),
            )
            .clone();

        let project_type = ProjectType::try_from((
            *args.get_one("lib").unwrap(),
            *args.get_one("bin").unwrap(),
            args.get_one::<String>("project-type"),
        ))
        .ok()?;

        let edition = CompilerEdition::try_from(
            args.get_one::<String>("edition")
                .map(|s| s.as_str())
                .unwrap_or("2024"),
        )
        .ok()?;

        let config = DefaultConfig::new(
            *args.get_one("verbose").unwrap(),
            *args.get_one("quiet").unwrap(),
        );

        Some(Self {
            project_type,
            edition,
            name,
            config,
        })
    }
}

#[derive(Debug)]
pub struct NewCommand {
    project_type: ProjectType,
    edition: CompilerEdition,
    name: String,
    config: DefaultConfig,
}

impl ClapCommandParser for NewCommand {
    fn command(am: &ArgManager) -> Command {
        Command::new("new")
            .about("Create new project")
            .arg(am.base("name", true, "Project name"))
            .arg(am.flag_long_val("type", false, "Project type (bin, lib)", "bin"))
            .arg(am.flag_short_val(
                "edition",
                false,
                "Rust edition (2015, 2018, 2021, 2024)",
                'e',
                "2024",
            ))
            .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
            .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
    }
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let name = args
            .get_one::<String>("name")
            .unwrap_or(
                &std::env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            )
            .clone();

        let project_type = ProjectType::try_from((
            *args.get_one("bin").unwrap(),
            *args.get_one("lib").unwrap(),
            args.get_one::<String>("type"),
        ))
        .unwrap();

        let edition = CompilerEdition::try_from(
            args.get_one::<String>("edition")
                .map(|s| s.as_str())
                .unwrap_or("2024"),
        )
        .unwrap();

        let config = DefaultConfig::new(
            *args.get_one("verbose").unwrap(),
            *args.get_one("quiet").unwrap(),
        );

        Some(Self {
            project_type,
            edition,
            name,
            config,
        })
    }
}

#[derive(Debug)]
pub struct AddCommandData {
    dependency_type: DependencyType,
    dependency_name: DependencyName,
    dependency_features: DependencyFeatures,
    target_platform: String,
    config: DefaultConfig,
}

#[derive(Debug)]
pub struct AddCommand {
    data: AddCommandData,
}

impl ClapCommandParser for AddCommand {
    fn command(am: &ArgManager) -> Command {
        Command::new("add")
            .about("Add dependency to project")
            .arg(am.base(
                "dependency",
                true,
                "Dependency name (optionally with version: name@1.0.0)",
            ))
            .arg(am.flag_long_val(
                "type",
                false,
                "Dependency type (dev, build, release)",
                "release",
            ))
            .arg(am.flag_long("features", false, "Features to enable (comma separated)"))
            .arg(am.flag_long_bool("default-features", false, "Enable default features", true))
            .arg(am.flag_long_bool(
                "no-default-features",
                false,
                "Disable default features",
                false,
            ))
            .arg(am.flag_long("target", false, "Target platform"))
            .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
            .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
    }
}

#[derive(Debug)]
pub struct StorageAddCommand {
    data: AddCommandData,
    storage_config: StorageConfig,
}

impl ClapCommandParser for StorageAddCommand {
    fn command(am: &ArgManager) -> Command {
        AddCommand::command(am).arg(am.flag_long("storage", true, "Path to storage"))
    }
}

pub fn command() -> Command {
    let am = ArgManager;

    Command::new("limp")
        .version("1.0")
        .author("Your Name")
        .about("Rust dependency management tool")
        .subcommand_required(true)
        .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
        .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
        .subcommand(InitCommand::command(&am))
        .subcommand(NewCommand::command(&am))
        .subcommand(AddCommand::command(&am))
        .subcommand(
            Command::new("storage")
                .about("Storage related commands")
                .subcommand_required(true)
                .subcommand(StorageAddCommand::command(&am)),
        )
}

#[cfg(test)]
mod test {
    mod init_command {
        use crate::{
            command::{command, DefaultConfig, InitCommand, LimpCommand},
            project::{CompilerEdition, ProjectType},
        };

        fn initialize_init_parse_helper_some(
            args: &[&str],
            sname: &str,
            sproject_type: ProjectType,
            sedition: CompilerEdition,
            sconfig: DefaultConfig,
        ) {
            let args = command().get_matches_from(args);
            let limp_command = LimpCommand::parse(&args);
            assert!(limp_command.is_some());
            let limp_command = limp_command.unwrap();

            assert!(matches!(limp_command, LimpCommand::Init(_)));

            if let LimpCommand::Init(InitCommand {
                project_type,
                edition,
                name,
                config,
            }) = limp_command
            {
                assert_eq!(sname, name);
                assert_eq!(sconfig.verbose, config.verbose);
                assert_eq!(sconfig.quiet, config.quiet);
                assert_eq!(sproject_type, project_type);
                assert_eq!(sedition, edition);
            }
        }
        fn initialize_init_parse_helper_none(args: &[&str]) {
            let args = command().get_matches_from(args);
            let limp_command = LimpCommand::parse(&args);
            assert!(limp_command.is_none());
        }

        #[test]
        fn initialize_init_parse_without_values_test() {
            initialize_init_parse_helper_some(
                &["limp", "init"],
                "limp",
                ProjectType::Bin,
                CompilerEdition::E2024,
                DefaultConfig::default(),
            );
        }

        #[test]
        fn initialize_init_parse_name_test() {
            let name = "blubli";
            initialize_init_parse_helper_some(
                &["limp", "init", name],
                name,
                ProjectType::Bin,
                CompilerEdition::E2024,
                DefaultConfig::default(),
            );
        }

        #[test]
        fn initialize_init_parse_all_valuess_test() {
            let name = "blubli2";
            initialize_init_parse_helper_some(
                &["limp", "init", "-vql", "-e", "15", name],
                name,
                ProjectType::Lib,
                CompilerEdition::E2015,
                DefaultConfig {
                    verbose: true,
                    quiet: true,
                },
            );
        }

        #[test]
        fn initialize_init_parse_incorrect_edition_test() {
            initialize_init_parse_helper_none(&["limp", "init", "-e", "1"]);
            initialize_init_parse_helper_none(&["limp", "init", "-e", "lskdjflksdjflsdkfj"]);
        }

        #[test]
        fn initialize_init_parse_incorrect_type_test() {
            initialize_init_parse_helper_none(&["limp", "init", "-lb"]);
            initialize_init_parse_helper_none(&["limp", "init", "-l", "--project-type", "bin"]);
            initialize_init_parse_helper_none(&["limp", "init", "-b", "--project-type", "lib"]);
            initialize_init_parse_helper_none(&["limp", "init", "--project-type", "sdlkjfslkdfj"]);
            initialize_init_parse_helper_none(&["limp", "init", "--project-type", "12"]);
            initialize_init_parse_helper_none(&["limp", "init", "-b", "--project-type", "sdf"]);
            initialize_init_parse_helper_none(&["limp", "init", "-l", "--project-type", "sdf"]);
            initialize_init_parse_helper_none(&["limp", "init", "-lb", "--project-type", "sdf"]);
        }
    }
    mod new_command {
        use crate::{
            command::{command, DefaultConfig, LimpCommand, NewCommand},
            project::{CompilerEdition, ProjectType},
        };

        fn initialize_new_parse_helper_some(
            args: &[&str],
            sname: &str,
            sproject_type: ProjectType,
            sedition: CompilerEdition,
            sconfig: DefaultConfig,
        ) {
            let args = command().get_matches_from(args);
            let limp_command = LimpCommand::parse(&args);
            assert!(limp_command.is_some());
            let limp_command = limp_command.unwrap();

            assert!(matches!(limp_command, LimpCommand::New(_)));

            if let LimpCommand::New(NewCommand {
                project_type,
                edition,
                name,
                config,
            }) = limp_command
            {
                assert_eq!(sname, name);
                assert_eq!(sconfig.verbose, config.verbose);
                assert_eq!(sconfig.quiet, config.quiet);
                assert_eq!(sproject_type, project_type);
                assert_eq!(sedition, edition);
            }
        }
        fn initialize_new_parse_helper_none(args: &[&str]) {
            let args = command().get_matches_from(args);
            let limp_command = LimpCommand::parse(&args);
            assert!(limp_command.is_none());
        }
    }
}
