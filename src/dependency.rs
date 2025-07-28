use crate::utils::{ValidProvider, ValidStr, ValidStrError, Validator};

#[derive(thiserror::Error, Debug)]
pub enum DependencyTypeError {
    #[error("Provided incompatible denendency types")]
    IncompatibleCargoTypes,
    #[error("Provided invalid stringify dependency type: {0}")]
    InvalidType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyFeaturesError {
    #[error("Provided incompatible denendency features")]
    IncompatibleFeatures,
    #[error("Provided invalid features {0}")]
    InvalidFeature(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyTargetTypeError {
    #[error("Provided incompatible denendency target: {0}")]
    InvalidType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyNameError {
    #[error("Provided invalid dependency name: {0}")]
    InvalidName(#[from] ValidStrError),
    #[error("Provided invalid dependency version: {0}")]
    InvalidVersion(#[from] DependencyVersionError),
}

#[derive(thiserror::Error, Debug)]
pub enum DependencyVersionError {
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Invalid parts: {0}")]
    InvalidParts(u8),
    #[error("Invalid numbers: {0}")]
    InvalidNumbers(String),
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum DependencyTargetType {
    #[default]
    All,
    Windows,
    Linux,
    Unix,
}

impl TryFrom<Option<&String>> for DependencyTargetType {
    type Error = DependencyTargetTypeError;
    fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {
        if let Some(target) = value {
            match target.as_str() {
                "windows" | "win" => Ok(DependencyTargetType::Windows),
                "linux" | "lin" => Ok(DependencyTargetType::Linux),
                "unix" | "ux" => Ok(DependencyTargetType::Unix),
                _ => Err(DependencyTargetTypeError::InvalidType(target.to_string())),
            }
        } else {
            Ok(DependencyTargetType::All)
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum DependencyType {
    Dev,
    Build,
    #[default]
    Release,
}

impl TryFrom<(bool, bool, bool, Option<&String>)> for DependencyType {
    type Error = DependencyTypeError;
    fn try_from(value: (bool, bool, bool, Option<&String>)) -> Result<Self, Self::Error> {
        let (dev, build, release, str_type) = value;

        if [dev, build, release].into_iter().filter(|&b| b).count() > 1 {
            return Err(DependencyTypeError::IncompatibleCargoTypes);
        }

        match str_type {
            Some(str_type) => match str_type.as_str() {
                "dev" if build || release => Err(DependencyTypeError::IncompatibleCargoTypes),
                "build" if dev || release => Err(DependencyTypeError::IncompatibleCargoTypes),
                "release" if dev || build => Err(DependencyTypeError::IncompatibleCargoTypes),

                "dev" | "d" => Ok(DependencyType::Dev),
                "build" | "b" => Ok(DependencyType::Build),
                "release" | "r" => Ok(DependencyType::Release),

                _ => Err(DependencyTypeError::InvalidType(str_type.to_string())),
            },
            None => {
                if dev {
                    Ok(DependencyType::Dev)
                } else if build {
                    Ok(DependencyType::Build)
                } else {
                    Ok(DependencyType::default())
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DependencyFeatures {
    default_features: bool,
    custom_features: Option<Vec<String>>,
}

impl DependencyFeatures {
    pub fn new(default_features: bool, custom_features: Option<Vec<String>>) -> Self {
        Self {
            default_features,
            custom_features,
        }
    }
}

impl Default for DependencyFeatures {
    fn default() -> Self {
        Self {
            default_features: true,
            custom_features: None,
        }
    }
}

impl TryFrom<(bool, bool, Option<&String>)> for DependencyFeatures {
    type Error = DependencyFeaturesError;
    fn try_from(value: (bool, bool, Option<&String>)) -> Result<Self, Self::Error> {
        let (defaultf, nodefaultf, features) = value;

        if defaultf && nodefaultf {
            return Err(DependencyFeaturesError::IncompatibleFeatures);
        }

        let default_features = !nodefaultf;

        let mut dependency_features = Self {
            default_features,
            custom_features: None,
        };

        if let Some(features) = features {
            let feature_list = features.split(',').collect::<Vec<&str>>();
            if feature_list.len() > 1 {
                if feature_list.iter().any(|&f| !is_valid_name(f)) {
                    return Err(DependencyFeaturesError::InvalidFeature(
                        features.to_string(),
                    ));
                }

                dependency_features.custom_features = Some(
                    feature_list
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<String>>(),
                );
            }
        }

        Ok(dependency_features)
    }
}

// name or name@version
#[derive(Debug, PartialEq, Eq)]
pub struct DependencyName {
    name: ValidStr,
    version: DependencyVersion,
}

impl TryFrom<&str> for DependencyName {
    type Error = DependencyNameError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let full_name = value.split('@').collect::<Vec<&str>>();

        let mut dependency_name = DependencyName {
            name: String::new(),
            version: DependencyVersion::default(),
        };

        match full_name.len() {
            1 => {
                let name = full_name[0];
                if is_valid_name(name) {
                    dependency_name.name = name.to_string()
                } else {
                    return Err(DependencyNameError::InvalidName(value.to_string()));
                }
            }
            2 => {
                let name = full_name[0];
                if is_valid_name(name) {
                    dependency_name.name = name.to_string()
                } else {
                    return Err(DependencyNameError::InvalidName(value.to_string()));
                }
                let version_str = DependencyVersion::try_from(full_name[2])?;
                dependency_name.version = version_str;
            }
            _ => return Err(DependencyNameError::InvalidName(value.to_string())),
        }

        Ok(dependency_name)
    }
}

impl DependencyName {
    fn parse_name(name: &str) -> Result<ValidStr, DependencyNameError> {
        Ok(ValidStr::new(name)?)
    }
    fn parse_version(version: &str) -> Result<DependencyVersion, DependencyVersionError> {}
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: DependencyVersion::default(),
        }
    }
    pub fn new_with_version(name: &str, version: DependencyVersion) -> Self {
        Self {
            name: name.to_string(),
            version,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum DependencyVersion {
    #[default]
    Latest,
    Custom {
        release: u32,
        major: u32,
        minor: u32,
    },
}

impl Validator for DependencyVersion {
    type Error = DependencyVersionError;
    fn check(&self) -> Result<(), Self::Error> {
        match self {
            DependencyVersion::Latest => Ok(()),
            DependencyVersion::Custom {
                release,
                major,
                minor,
            } => {
                if *release == 0 || *major == 0 || *minor == 0 {
                    Err(DependencyVersionError::InvalidNumbers(format!(
                        "release: {}, major: {}, minor: {}",
                        release, major, minor
                    )))
                } else {
                    Ok(())
                }
            }
        }
    }
}
impl<S: AsRef<str>> ValidProvider<S> for DependencyVersion {
    fn new(value: S) -> Result<Self, Self::Error> {
        let vs = ValidStr::from(value);
        vs.check()?;
    }
} // ValidProvider

impl TryFrom<&str> for DependencyVersion {
    type Error = DependencyVersionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(DependencyVersion::Latest);
        }

        let parts: Vec<&str> = value.split('.').collect();

        match parts.len() {
            1 => {
                let release = parts[0].parse::<u32>()?;
                Ok(DependencyVersion::Custom {
                    release,
                    major: 0,
                    minor: 0,
                })
            }
            2 => {
                let release = parts[0].parse::<u32>()?;
                let major = parts[1].parse::<u32>()?;
                Ok(DependencyVersion::Custom {
                    release,
                    major,
                    minor: 0,
                })
            }
            3 => {
                let release = parts[0].parse::<u32>()?;
                let major = parts[1].parse::<u32>()?;
                let minor = parts[2].parse::<u32>()?;
                Ok(DependencyVersion::Custom {
                    release,
                    major,
                    minor,
                })
            }
            _ => Err(DependencyVersionError::InvalidParts(parts.len() as u8)),
        }
    }
}

fn is_valid_name(name: &str) -> bool {
    !(name.contains(".") || name.chars().next().unwrap().is_ascii_digit())
}

// I dont know how to create scalable etc.
// TODO: think about dependency architecture

#[derive(Debug)]
pub struct Dependency {
    name: DependencyName,
    features: DependencyFeatures,

    dependency_type: DependencyType,
}

// comma separated names without any field
pub struct InputDependencies(String);

// TODO: from input with storage and cratesio intgration
pub struct OutputDependencies {
    items: Vec<Dependency>,
}
