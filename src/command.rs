use std::path::PathBuf;

use clap::{ArgMatches, Command};

use crate::{
    arg::ArgManager,
    dependency::{DependencyFeatures, DependencyName, DependencyTargetType, DependencyType},
    project::{CompilerEdition, ProjectType},
    utils::{ValidProvider, ValidStr},
};

pub trait ClapCommandProvider {
    fn command(am: &ArgManager) -> Command;
}

pub trait ClapCommandParser: Sized {
    // TODO: rewrite to LimpRestlt<Self>
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let _ = args;
        None
    }
}

#[derive(Debug)]
pub struct MainApplication {
    limp_command: LimpCommand,
    config: LimpConfig,
}

impl ClapCommandProvider for MainApplication {
    fn command(am: &ArgManager) -> Command {
        Command::new("limp")
            .version("1.0")
            .author("Your Name")
            .about("Rust dependency management tool")
            .subcommand_required(true)
            .arg(am.flag_short_bool("verbose", false, "Verbose output", 'v', false))
            .arg(am.flag_short_bool("quiet", false, "Quiet mode", 'q', false))
            .subcommand(InitCommand::command(am))
            .subcommand(NewCommand::command(am))
            .subcommand(AddCommand::command(am))
            .subcommand(
                Command::new("storage")
                    .about("Storage related commands")
                    .subcommand_required(true)
                    .subcommand(StorageAddCommand::command(am)),
            )
    }
}
impl ClapCommandParser for MainApplication {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let limp_command = LimpCommand::from_matches(args)?;
        let config = LimpConfig::new(*args.get_one("verbose")?, *args.get_one("quiet")?);

        Some(Self {
            limp_command,
            config,
        })
    }
}

impl MainApplication {
    pub fn get_command(&self) -> &LimpCommand {
        &self.limp_command
    }
    pub fn get_config(&self) -> &LimpConfig {
        &self.config
    }
}

#[derive(Debug)]
pub enum LimpCommand {
    Init(InitCommand),
    New(NewCommand),
    Add(AddCommand),
    StorageAdd(StorageAddCommand),
}

impl ClapCommandParser for LimpCommand {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
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
pub struct LimpConfig {
    verbose: bool,
    quiet: bool,
}

impl LimpConfig {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }
}

#[derive(Debug)]
pub struct StorageConfig {
    path_to_storage: PathBuf,
}

impl StorageConfig {
    pub fn new(path_to_storage: PathBuf) -> Self {
        Self { path_to_storage }
    }
}

#[derive(Debug)]
pub struct InitCommand {
    name: ValidStr,

    project_type: ProjectType,
    edition: CompilerEdition,
}

impl ClapCommandProvider for InitCommand {
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
    }
}
impl ClapCommandParser for InitCommand {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        // String and &str -> ok
        // &String and &str -> bruh
        let name = ValidStr::new(
            args.get_one::<String>("name")
                .unwrap_or(
                    &std::env::current_dir()
                        .unwrap()
                        .file_name()?
                        .to_str()?
                        .to_string(),
                )
                .clone(),
        )
        .ok()?;

        let project_type = ProjectType::try_from((
            *args.get_one("lib")?,
            *args.get_one("bin")?,
            args.get_one::<String>("project-type"),
        ))
        .ok()?;

        let edition = CompilerEdition::try_from(
            args.get_one::<String>("edition")
                .map(|s| s.as_str())
                .unwrap_or("2024"),
        )
        .ok()?;

        Some(Self {
            project_type,
            edition,
            name,
        })
    }
}

#[derive(Debug)]
pub struct NewCommand {
    name: ValidStr,

    project_type: ProjectType,
    edition: CompilerEdition,
}

impl ClapCommandProvider for NewCommand {
    fn command(am: &ArgManager) -> Command {
        Command::new("new")
            .about("Create new project")
            .arg(am.base("name", true, "Project name"))
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
    }
}
impl ClapCommandParser for NewCommand {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let name = ValidStr::new(
            args.get_one::<String>("name")
                .unwrap_or(
                    &std::env::current_dir()
                        .unwrap()
                        .file_name()?
                        .to_str()?
                        .to_string(),
                )
                .clone(),
        )
        .ok()?;

        let project_type = ProjectType::try_from((
            *args.get_one("lib")?,
            *args.get_one("bin")?,
            args.get_one::<String>("project-type"),
        ))
        .ok()?;

        let edition = CompilerEdition::try_from(
            args.get_one::<String>("edition")
                .map(|s| s.as_str())
                .unwrap_or("2024"),
        )
        .ok()?;

        Some(Self {
            project_type,
            edition,
            name,
        })
    }
}

#[derive(Debug)]
pub struct AddCommandData {
    dependency_type: DependencyType,
    dependency_name: DependencyName,
    dependency_features: DependencyFeatures,
    dependency_target: DependencyTargetType,
}

impl ClapCommandProvider for AddCommandData {
    fn command(am: &ArgManager) -> Command {
        Command::new("add")
            .about("Add dependency to project")
            .arg(am.base(
                "dependency",
                true,
                "Dependency name (optionally with version: name@<ver>)",
            ))
            .arg(am.flag_long_bool("dev", false, "Set dependency type to dev", false))
            .arg(am.flag_long_bool("build", false, "Set dependency type to build", false))
            .arg(am.flag_long_bool("release", false, "Set dependency type to release", false))
            .arg(am.flag_long("type", false, "Dependency type (dev, build, release)"))
            .arg(am.flag_short(
                "features",
                false,
                "Features to enable (comma separated)",
                'F',
            ))
            .arg(am.flag_long_bool("default-features", false, "Enable default features", false))
            .arg(am.flag_long_bool(
                "no-default-features",
                false,
                "Disable default features",
                false,
            ))
            .arg(am.flag_short("target", false, "Target platform", 't'))
    }
}

impl ClapCommandParser for AddCommandData {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let dependency_type = DependencyType::try_from((
            *args.get_one("dev")?,
            *args.get_one("build")?,
            *args.get_one("release")?,
            args.get_one::<String>("type"),
        ))
        .ok()?;

        let dependency_features = DependencyFeatures::try_from((
            *args.get_one("default-features")?,
            *args.get_one("no-default-features")?,
            args.get_one::<String>("features"),
        ))
        .ok()?;

        let dependency_name =
            DependencyName::try_from(args.get_one::<String>("dependency").map(|s| s.as_str())?)
                .ok()?;

        let dependency_target = DependencyTargetType::try_from(args.get_one("target")).ok()?;

        Some(Self {
            dependency_type,
            dependency_name,
            dependency_features,
            dependency_target,
        })
    }
}

#[derive(Debug)]
pub struct AddCommand {
    data: AddCommandData,
}

impl ClapCommandProvider for AddCommand {
    fn command(am: &ArgManager) -> Command {
        AddCommandData::command(am)
    }
}

impl ClapCommandParser for AddCommand {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        Some(Self {
            data: AddCommandData::from_matches(args)?,
        })
    }
}

#[derive(Debug)]
pub struct StorageAddCommand {
    data: AddCommandData,
    storage_config: StorageConfig,
}

impl ClapCommandProvider for StorageAddCommand {
    fn command(am: &ArgManager) -> Command {
        AddCommandData::command(am).arg(am.flag_short("storage-path", true, "Path to storage", 'P'))
    }
}

impl ClapCommandParser for StorageAddCommand {
    fn from_matches(args: &ArgMatches) -> Option<Self> {
        let storage_config = StorageConfig::new(PathBuf::from(args.get_one::<String>("storage")?));
        Some(Self {
            data: AddCommandData::from_matches(args)?,
            storage_config,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        arg::ArgManager,
        command::{ClapCommandParser, ClapCommandProvider, MainApplication},
    };

    fn get_main_application(args: &[&str]) -> Option<MainApplication> {
        let am = &ArgManager;
        let args = MainApplication::command(am).get_matches_from(args);
        MainApplication::from_matches(&args)
    }
    mod init_command {
        use crate::{
            command::{test::get_main_application, InitCommand, LimpCommand},
            project::{CompilerEdition, ProjectType},
            utils::{ValidProvider, ValidStr},
        };

        fn initialize_init_parse_helper_some(
            args: &[&str],
            sname: ValidStr,
            sproject_type: ProjectType,
            sedition: CompilerEdition,
        ) {
            let app = get_main_application(args);
            assert!(app.is_some());
            let app = app.unwrap();

            assert!(matches!(app.limp_command, LimpCommand::Init(_)));

            if let LimpCommand::Init(InitCommand {
                project_type,
                edition,
                name,
            }) = app.limp_command
            {
                assert_eq!(sname, name);
                assert_eq!(sproject_type, project_type);
                assert_eq!(sedition, edition);
            }
        }
        fn initialize_init_parse_helper_none(args: &[&str]) {
            let app = get_main_application(args);
            assert!(app.is_none());
        }

        #[test]
        fn initialize_init_parse_without_values_test() {
            initialize_init_parse_helper_some(
                &["limp", "init"],
                ValidStr::new("limp".to_string()).unwrap(),
                ProjectType::Bin,
                CompilerEdition::E2024,
            );
        }

        #[test]
        fn initialize_init_parse_name_test() {
            let name = "blubli";
            initialize_init_parse_helper_some(
                &["limp", "init", name],
                ValidStr::new(name.to_string()).unwrap(),
                ProjectType::Bin,
                CompilerEdition::E2024,
            );
        }

        #[test]
        fn initialize_init_parse_all_valuess_test() {
            let name = "blubli2";
            initialize_init_parse_helper_some(
                &["limp", "init", "-l", "-e", "15", name],
                ValidStr::new(name.to_string()).unwrap(),
                ProjectType::Lib,
                CompilerEdition::E2015,
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
            command::{test::get_main_application, LimpCommand, NewCommand},
            project::{CompilerEdition, ProjectType},
            utils::ValidStr,
        };

        fn initialize_new_parse_helper_some(
            args: &[&str],
            sname: ValidStr,
            sproject_type: ProjectType,
            sedition: CompilerEdition,
        ) {
            let app = get_main_application(args);
            assert!(app.is_some());
            let app = app.unwrap();

            assert!(matches!(app.limp_command, LimpCommand::New(_)));

            if let LimpCommand::New(NewCommand {
                project_type,
                edition,
                name,
            }) = app.limp_command
            {
                assert_eq!(sname, name);
                assert_eq!(sproject_type, project_type);
                assert_eq!(sedition, edition);
            }
        }
        fn initialize_new_parse_helper_none(args: &[&str]) {
            let app = get_main_application(args);
            assert!(app.is_none());
        }
        #[test]
        fn initialize_new_parse_name_test() {
            let name = "blubli";
            initialize_new_parse_helper_some(
                &["limp", "new", name],
                ValidStr::from(name),
                ProjectType::Bin,
                CompilerEdition::E2024,
            );
        }

        #[test]
        fn initialize_new_parse_all_values_test() {
            let name = "blubli_new";
            initialize_new_parse_helper_some(
                &["limp", "new", "-l", "-e", "21", name],
                ValidStr::from(name),
                ProjectType::Lib,
                CompilerEdition::E2021,
            );
        }
        #[test]
        fn initialize_new_parse_invalid_edition_test() {
            let name = "blubli";
            initialize_new_parse_helper_none(&["limp", "new", "-e", "bruh", name]);
            initialize_new_parse_helper_none(&["limp", "new", "-e", "1231", name]);
            initialize_new_parse_helper_none(&["limp", "new", "-e", "2014", name]);
        }
        #[test]
        fn initialize_new_parse_invalid_project_type_test() {
            let name = "blubli";
            initialize_new_parse_helper_none(&["limp", "new", "-lb", name]);
            initialize_new_parse_helper_none(&["limp", "new", "-l", "--project-type", "bin", name]);
            initialize_new_parse_helper_none(&["limp", "new", "-b", "--project-type", "lib", name]);
            initialize_new_parse_helper_none(&[
                "limp",
                "new",
                "--project-type",
                "sdlkjfslkdfj",
                name,
            ]);
            initialize_new_parse_helper_none(&["limp", "new", "--project-type", "12", name]);
            initialize_new_parse_helper_none(&["limp", "new", "-b", "--project-type", "sdf", name]);
            initialize_new_parse_helper_none(&["limp", "new", "-l", "--project-type", "sdf", name]);
            initialize_new_parse_helper_none(&[
                "limp",
                "new",
                "-lb",
                "--project-type",
                "sdf",
                name,
            ]);
        }
    }
    mod add_command {
        use crate::{
            command::{test::get_main_application, AddCommand, LimpCommand},
            dependency::{
                DependencyFeatures, DependencyName, DependencyTargetType, DependencyType,
            },
        };

        fn initialize_add_parse_helper_some(
            args: &[&str],
            dependency_name: DependencyName,
            dependency_features: DependencyFeatures,
            dependency_type: DependencyType,
            dependency_target: DependencyTargetType,
        ) {
            let app = get_main_application(args);
            assert!(app.is_some());
            let app = app.unwrap();

            assert!(matches!(app.limp_command, LimpCommand::Add(_)));

            if let LimpCommand::Add(AddCommand { data }) = app.limp_command {
                assert_eq!(data.dependency_name, dependency_name);
                assert_eq!(data.dependency_features, dependency_features);
                assert_eq!(data.dependency_type, dependency_type);
                assert_eq!(data.dependency_target, dependency_target);
            }
        }

        fn initialize_add_parse_helper_none(args: &[&str]) {
            let app = get_main_application(args);
            assert!(app.is_none())
        }

        #[test]
        fn initialize_add_parse_only_name_test() {
            let name = "add_test";
            initialize_add_parse_helper_some(
                &["limp", "add", name],
                DependencyName::new(name),
                DependencyFeatures::default(),
                DependencyType::default(),
                DependencyTargetType::default(),
            );
        }

        #[test]
        fn initialize_add_parse_all_values_test() {
            let name = "add_test2";
            let dependency_name = DependencyName::try_from(name);
            assert!(dependency_name.is_ok());
            let dependency_name = dependency_name.unwrap();
            initialize_add_parse_helper_some(
                &[
                    "limp",
                    "add",
                    "--no-default-features",
                    "-F",
                    "f1,f2",
                    "--build",
                    "-t",
                    "linux",
                    name,
                ],
                dependency_name,
                DependencyFeatures::new(false, Some(vec!["f1".to_string(), "f2".to_string()])),
                DependencyType::Build,
                DependencyTargetType::Linux,
            );
        }

        #[test]
        fn initialize_add_parse_incorrect_features_test() {
            let name = "add_test_features";
            initialize_add_parse_helper_none(&[
                "limp",
                "add",
                "--no-default-features",
                "--default-features",
                name,
            ]);
            initialize_add_parse_helper_none(&["limp", "add", "-F", "1,fe2", name]);
            initialize_add_parse_helper_none(&["limp", "add", "-F", "fe1,1,fe2", name]);
            initialize_add_parse_helper_none(&["limp", "add", "-F", "fe1,fe1.2", name]);
        }

        #[test]
        fn initialize_add_parse_incorrect_name_test() {
            initialize_add_parse_helper_none(&["limp", "add", ".name"]);
            initialize_add_parse_helper_none(&["limp", "add", "123adf"]);
        }
    }
}
