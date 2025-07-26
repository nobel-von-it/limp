#[derive(Default, Debug)]
pub enum DependencyType {
    Dev,
    Build,
    #[default]
    Release,
}

#[derive(Debug)]
pub struct DependencyFeatures {
    default_features: bool,
    custom_features: Option<String>,
}

impl Default for DependencyFeatures {
    fn default() -> Self {
        Self {
            default_features: true,
            custom_features: None,
        }
    }
}

// name or name@version
#[derive(Debug)]
pub struct DependencyName {
    name: String,
    version: DependencyVersion,
}

#[derive(Debug)]
pub enum DependencyVersion {
    Latest,
    Custom {
        release: u32,
        major: u32,
        minor: u32,
    },
}
