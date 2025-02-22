use serde::{Deserialize, Serialize};

use crate::error::LimpError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoDependency {
    #[serde(rename = "crate")]
    pub crate_info: Crate,
    pub versions: Vec<serde_json::Value>,
}
impl CratesIoDependency {
    pub fn from_cratesio(name: &str) -> Result<Self, LimpError> {
        let url = format!("https://crates.io/api/v1/crates/{}", name);
        let res = ureq::get(&url)
            .set("User-Agent", "limp/0.2.1")
            .call()
            .map_err(|e| LimpError::HttpError(Box::new(e)))?;
        let body = res.into_string()?;
        Ok(serde_json::from_str(&body)?)
    }
    pub fn get_all_versions(&self) -> Vec<Version> {
        self.versions
            .iter()
            .filter_map(|v| serde_json::from_value::<Version>(v.clone()).ok())
            .collect()
    }
    pub fn get_features(&self, id: u64) -> Option<Vec<String>> {
        if let Ok(version) = self.get_version(id) {
            return version.get_features();
        }
        None
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub max_version: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub features: Option<serde_json::Value>,
    pub num: String,
}
impl Version {
    pub fn get_features(&self) -> Option<Vec<String>> {
        if let Some(features) = &self.features {
            if let Some(obj) = features.as_object() {
                return Some(obj.keys().cloned().collect());
            }
        }
        None
    }
}
