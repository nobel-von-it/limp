use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    crates::CratesIoDependency,
    error::LimpError,
    files::{self, add_to_snippets_dir},
    parser::SnippetEntity,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JsonDependency {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub features: Option<Vec<String>>,
    #[serde(default)]
    pub path_to_snippet: Option<String>,
}

impl std::fmt::Display for JsonDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // name = version
        // or
        // name {"version" = version, "features" = [...features]}
        if let Some(features) = &self.features {
            let deps = features
                .iter()
                .map(|f| format!("\"{f}\""))
                .collect::<Vec<String>>()
                .join(", ");
            let res = format!(
                "{} = (version = \"{}\", features = [{}])",
                &self.name, &self.version, deps
            )
            .replace("(", "{")
            .replace(")", "}");
            write!(f, "{}", &res)
        } else {
            write!(f, "{} = \"{}\"", &self.name, &self.version)
        }
    }
}

impl JsonDependency {
    pub fn new(name: &str) -> Result<Self, LimpError> {
        let crateiodep = CratesIoDependency::from_cratesio(name)?;
        Ok(Self {
            name: name.to_string(),
            version: crateiodep.get_version(0)?.num.clone(),
            features: None,
            path_to_snippet: None,
        })
    }
    pub fn new_full(
        name: &str,
        version: Option<&str>,
        features: Option<&[String]>,
        path_to_snippet: Option<&str>,
    ) -> Result<Self, LimpError> {
        let crateiodep = CratesIoDependency::from_cratesio(name)?;

        let mut result_path = None;

        if let Some(path) = path_to_snippet {
            let path = PathBuf::from(path);
            if !path.exists() {
                return Err(LimpError::SnippetNotFound(path.display().to_string()));
            }
            let se = SnippetEntity::from_file(path)?;
            if let Ok(p) = add_to_snippets_dir(name, se.to_string().as_str()) {
                result_path = Some(p);
            }
        }

        if let Some(version) = version {
            let all_versions = crateiodep.get_all_versions();
            let finded_version = all_versions
                .iter()
                .find(|v| v.num == version)
                .ok_or_else(|| LimpError::VersionNotFound(format!("{}/{}", name, version)))?;

            if let Some(features) = features {
                let finded_features = finded_version.get_features().ok_or_else(|| {
                    LimpError::IncompatibleFeatures(format!("{}/{}", name, version))
                })?;

                if !features.iter().all(|f| finded_features.contains(f)) {
                    return Err(LimpError::IncompatibleFeatures(format!(
                        "{}/{}",
                        name, version
                    )));
                }
            }
        }

        Ok(Self {
            name: name.to_string(),
            version: version
                .unwrap_or(&crateiodep.get_version(0)?.num)
                .to_string(),
            features: features.map(|f| f.to_vec()),
            path_to_snippet: result_path,
        })
    }
    pub fn update(&mut self) -> Result<(), LimpError> {
        let crateiodep = CratesIoDependency::from_cratesio(&self.name)?;
        self.version = crateiodep.get_version(0)?.num.clone();
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct JsonStorage {
    #[serde(default)]
    pub dependencies: HashMap<String, JsonDependency>,
}

impl JsonStorage {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<JsonStorage, LimpError> {
        let file = files::open(path)?;
        Ok(serde_json::from_reader(file).unwrap_or(JsonStorage::default()))
    }
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), LimpError> {
        let file = files::open(path)?;
        file.set_len(0)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn add(&mut self, dep: JsonDependency) {
        self.dependencies.insert(dep.name.clone(), dep);
    }

    pub fn remove(&mut self, name: &str) {
        self.dependencies.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&JsonDependency> {
        self.dependencies.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut JsonDependency> {
        self.dependencies.get_mut(name)
    }
}
