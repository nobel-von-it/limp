use std::path::PathBuf;

use clap::{ArgMatches, Command};

use crate::{
    arg::ArgManager,
    dependency::{RustDependencyFeatures, RustDependencyName, RustDependencyType},
    project::{CompilerEdition, ProjectType},
};

pub trait ClapCommandParser: Sized {
    fn command(am: &ArgManager) -> Command;
    // TODO: rewrite to LimpRestlt<Self>
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        None
    }
}

pub enum LimpCommand {
    Initialize(InitializeCommand),
    Storage(StorageCommand),
}

pub enum InitializeCommand {
    Init(InitCommand),
    New(NewCommand),
    Add(AddCommand),
}

pub enum StorageCommand {
    Add(StorageAddCommand),
}

pub struct DefaultConfig {
    verbose: bool,
    quiet: bool,
}

impl DefaultConfig {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }
}

pub struct StorageConfig {
    path_to_storage: PathBuf,
}

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
            .arg(am.base("name", true, "Project name"))
            .arg(am.flag_short_bool(
                "bin",
                false,
                "Set project type to binary (default)",
                'b',
                true,
            ))
            .arg(am.flag_short_bool("lib", false, "Set project type to lib", 'l', false))
            .arg(am.flag_long_val("project-type", false, "Project type (bin, lib)", "bin"))
            .arg(am.flag_long_val(
                "edition",
                false,
                "Rust edition (2015, 2018, 2021, 2024)",
                "2024",
            ))
            .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
            .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
    }
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        // String and &str -> ok
        // &String and &str -> bruh
        let name = args.get_one::<String>("name").unwrap_or(
            &std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

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
            .arg(am.flag_long_val(
                "edition",
                false,
                "Rust edition (2015, 2018, 2021, 2024)",
                "2024",
            ))
            .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
            .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
    }
}

pub struct AddCommand {
    dependency_type: RustDependencyType,
    dependency_name: RustDependencyName,
    dependency_features: RustDependencyFeatures,
    target_platform: String,
    config: DefaultConfig,
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

pub struct StorageAddCommand {
    dependency_type: RustDependencyType,
    dependency_name: RustDependencyName,
    dependency_features: RustDependencyFeatures,
    target_platform: String,
    config: DefaultConfig,
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
