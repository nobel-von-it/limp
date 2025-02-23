//! # Dependency Storage Module
//!
//! This module provides functionality for managing dependencies and their metadata.
//! It includes structures for storing dependency information (`JsonDependency`) and
//! managing a collection of dependencies (`JsonStorage`).

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

/// Represents a dependency with its metadata.
///
/// This struct stores information about a dependency, including its name, version,
/// optional features, and an optional path to a code snippet.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JsonDependency {
    /// The name of the dependency.
    pub name: String,
    /// The version of the dependency.
    pub version: String,
    /// Optional features enabled for the dependency.
    #[serde(default)]
    pub features: Option<Vec<String>>,
    /// Optional path to a code snippet associated with the dependency.
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
    /// Creates a new `JsonDependency` from a crate name.
    ///
    /// This function fetches the latest version of the crate from crates.io
    /// and initializes a `JsonDependency` with the default version.
    ///
    /// # Arguments
    /// * `name` - The name of the crate.
    ///
    /// # Returns
    /// - `Ok(JsonDependency)` if the crate is successfully fetched.
    /// - `Err(LimpError)` if the crate cannot be found or an error occurs.
    pub fn new(name: &str) -> Result<Self, LimpError> {
        let crateiodep = CratesIoDependency::from_cratesio(name)?;
        Ok(Self {
            name: name.to_string(),
            version: crateiodep.get_version(0)?.num.clone(),
            features: None,
            path_to_snippet: None,
        })
    }

    /// Creates a new `JsonDependency` with full metadata.
    ///
    /// This function allows specifying the version, features, and snippet path
    /// for the dependency. It also validates the version and features against crates.io.
    ///
    /// # Arguments
    /// * `name` - The name of the crate.
    /// * `version` - The version of the crate (optional).
    /// * `features` - The features to enable (optional).
    /// * `path_to_snippet` - The path to a code snippet (optional).
    ///
    /// # Returns
    /// - `Ok(JsonDependency)` if the dependency is successfully created.
    /// - `Err(LimpError)` if the version or features are invalid or an error occurs.
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

    /// Updates the dependency to the latest version from crates.io.
    ///
    /// This function fetches the latest version of the crate from crates.io
    /// and updates the `version` field of the dependency.
    ///
    /// # Returns
    /// - `Ok(())` if the update is successful.
    /// - `Err(LimpError)` if the crate cannot be found or an error occurs.
    pub fn update(&mut self) -> Result<(), LimpError> {
        let crateiodep = CratesIoDependency::from_cratesio(&self.name)?;
        self.version = crateiodep.get_version(0)?.num.clone();
        Ok(())
    }
}

/// Represents a collection of dependencies stored in a JSON file.
///
/// This struct manages a collection of `JsonDependency` objects and provides
/// functionality for loading, saving, and manipulating dependencies.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct JsonStorage {
    /// A map of dependencies, keyed by their names.
    #[serde(default)]
    pub dependencies: HashMap<String, JsonDependency>,
}

impl JsonStorage {
    /// Loads dependencies from a JSON file.
    ///
    /// This function reads a JSON file and deserializes it into a `JsonStorage` object.
    /// If the file does not exist or is invalid, it returns a default `JsonStorage`.
    ///
    /// # Arguments
    /// * `path` - The path to the JSON file.
    ///
    /// # Returns
    /// - `Ok(JsonStorage)` if the file is successfully loaded.
    /// - `Err(LimpError)` if an error occurs.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<JsonStorage, LimpError> {
        let file = files::open(path)?;
        Ok(serde_json::from_reader(file).unwrap_or(JsonStorage::default()))
    }

    /// Saves dependencies to a JSON file.
    ///
    /// This function serializes the `JsonStorage` object and writes it to a JSON file.
    ///
    /// # Arguments
    /// * `path` - The path to the JSON file.
    ///
    /// # Returns
    /// - `Ok(())` if the file is successfully saved.
    /// - `Err(LimpError)` if an error occurs.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), LimpError> {
        let file = files::open(path)?;
        // Clear the file before writing
        // If append is used here, it will broke the file and save wrong data
        file.set_len(0)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    /// Adds a dependency to the storage.
    ///
    /// This function inserts a `JsonDependency` into the `dependencies` map.
    /// If a dependency with the same name already exists, it will be overwritten.
    /// This is just wrapper for `insert()` function
    ///
    /// # Arguments
    /// * `dep` - The dependency to add.
    pub fn add(&mut self, dep: JsonDependency) {
        self.dependencies.insert(dep.name.clone(), dep);
    }

    /// Removes a dependency from the storage.
    ///
    /// This function removes a dependency from the `dependencies` map by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the dependency to remove.
    pub fn remove(&mut self, name: &str) {
        self.dependencies.remove(name);
    }

    /// Retrieves a dependency by its name.
    ///
    /// This function returns a reference to a `JsonDependency` if it exists in the storage.
    /// This is just wrapper for `get()` function
    ///
    /// # Arguments
    /// * `name` - The name of the dependency to retrieve.
    ///
    /// # Returns
    /// - `Some(&JsonDependency)` if the dependency exists.
    /// - `None` if the dependency does not exist.
    pub fn get(&self, name: &str) -> Option<&JsonDependency> {
        self.dependencies.get(name)
    }

    /// Retrieves a mutable reference to a dependency by its name.
    ///
    /// This function returns a mutable reference to a `JsonDependency` if it exists in the storage.
    /// This is just wrapper for `get_mut()` function
    ///
    /// # Arguments
    /// * `name` - The name of the dependency to retrieve.
    ///
    /// # Returns
    /// - `Some(&mut JsonDependency)` if the dependency exists.
    /// - `None` if the dependency does not exist.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut JsonDependency> {
        self.dependencies.get_mut(name)
    }
}
