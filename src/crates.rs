//! # Crates.io Dependency Module
//!
//! This module provides functionality for interacting with the [crates.io](https://crates.io/) API.
//! It allows fetching crate information, versions, and features from crates.io.

use serde::{Deserialize, Serialize};

use crate::error::LimpError;

/// Represents a crate and its versions fetched from crates.io.
///
/// This struct contains metadata about a crate, including its name and all available versions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoDependency {
    /// Metadata about the crate.
    #[serde(rename = "crate")]
    pub crate_info: Crate,
    /// List of versions for the crate.
    pub versions: Vec<serde_json::Value>,
}
impl CratesIoDependency {
    /// Fetches crate metadata from crates.io.
    ///
    /// This function sends an HTTP request to crates.io to retrieve metadata about the specified crate.
    ///
    /// # Arguments
    /// * `name` - The name of the crate to fetch.
    ///
    /// # Returns
    /// - `Ok(CratesIoDependency)` if the request is successful.
    /// - `Err(LimpError)` if the request fails or the response cannot be parsed.
    pub fn from_cratesio(name: &str) -> Result<Self, LimpError> {
        let url = format!("https://crates.io/api/v1/crates/{}", name);
        let res = ureq::get(&url)
            .set("User-Agent", "limp/0.2.1")
            .call()
            .map_err(|e| LimpError::HttpError(Box::new(e)))?;
        let body = res.into_string()?;
        Ok(serde_json::from_str(&body)?)
    }

    /// Retrieves all versions of the crate.
    ///
    /// This function parses the `versions` field and returns a list of `Version` objects.
    ///
    /// # Returns
    /// A `Vec<Version>` containing all versions of the crate. The crate has at least one version.
    pub fn get_all_versions(&self) -> Vec<Version> {
        self.versions
            .iter()
            .filter_map(|v| serde_json::from_value::<Version>(v.clone()).ok())
            .collect()
    }

    /// Retrieves the features for a specific version of the crate.
    ///
    /// # Arguments
    /// * `id` - The index of the version in the `versions` list.
    ///
    /// # Returns
    /// - `Some(Vec<String>)` if the version has features.
    /// - `None` if the version has no features or the version ID is invalid.
    pub fn get_features(&self, id: u64) -> Option<Vec<String>> {
        if let Ok(version) = self.get_version(id) {
            return version.get_features();
        }
        None
    }

    /// Retrieves a specific version of the crate by its index.
    ///
    /// # Arguments
    /// * `id` - The index of the version in the `versions` list.
    ///
    /// # Returns
    /// - `Ok(Version)` if the version exists.
    /// - `Err(LimpError)` if the version ID is invalid.
    pub fn get_version(&self, id: u64) -> Result<Version, LimpError> {
        if let Some(value) = self.versions.get(id as usize) {
            let version = serde_json::from_value(value.clone())?;
            return Ok(version);
        }
        Err(LimpError::VersionNotFound(format!(
            "{}/{}",
            self.crate_info.name, id
        )))
    }
}

/// Metadata about a crate.
///
/// This struct contains basic information about a crate, such as its name and the latest version.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    /// The name of the crate.
    pub name: String,
    /// The latest version of the crate.
    pub max_version: String,
}
/// Represents a specific version of a crate.
///
/// This struct contains information about a version, including its number and available features.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    /// The name of the crate.
    #[serde(rename = "crate")]
    pub crate_name: String,
    /// Features available in this version (if any).
    pub features: Option<serde_json::Value>,
    /// The version number.
    pub num: String,
}
impl Version {
    /// Retrieves the features for this version of the crate.
    ///
    /// # Returns
    /// - `Some(Vec<String>)` if the version has features.
    /// - `None` if the version has no features.
    pub fn get_features(&self) -> Option<Vec<String>> {
        if let Some(features) = &self.features {
            if let Some(obj) = features.as_object() {
                return Some(obj.keys().cloned().collect());
            }
        }
        None
    }
}
