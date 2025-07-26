#[derive(Default)]
pub enum RustDependencyType {
    Dev,
    Build,
    #[default]
    Release,
}

pub struct RustDependencyFeatures {
    default_features: bool,
    custom_features: Option<String>,
}

impl Default for RustDependencyFeatures {
    fn default() -> Self {
        Self {
            default_features: true,
            custom_features: None,
        }
    }
}

// name or name@version
pub struct RustDependencyName {
    name: String,
    version: RustDependencyVersion,
}

pub enum RustDependencyVersion {
    Latest,
    Custom {
        release: u32,
        major: u32,
        minor: u32,
    },
}
