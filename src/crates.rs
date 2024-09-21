use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullCrateInfo {
    #[serde(rename = "crate")]
    pub crate_info: Crate,
    versions: Vec<serde_json::Value>,
}
impl FullCrateInfo {
    // TODO: rewrite to Result and use ?
    pub fn get_from_cratesio(name: &str) -> Option<Self> {
        let url = format!("https://crates.io/api/v1/crates/{}", name);
        if let Ok(res) = ureq::get(&url).set("User-Agent", "limp/0.1.2").call() {
            if let Ok(body) = res.into_string() {
                if let Ok(crate_info) = serde_json::from_str::<FullCrateInfo>(&body) {
                    return Some(crate_info);
                }
            }
        }
        None
    }
    pub fn get_all_versions(&self) -> Vec<Version> {
        let mut vs = vec![];
        for v in self.versions.iter() {
            if let Ok(v) = serde_json::from_value::<Version>(v.clone()) {
                vs.push(v)
            }
        }
        vs
    }
    pub fn get_features(&self, id: u64) -> Option<Vec<String>> {
        if let Some(version) = self.get_version(id) {
            return version.get_features();
        }
        None
    }
    pub fn get_version(&self, id: u64) -> Option<Version> {
        if let Some(value) = self.versions.get(id as usize) {
            if let Ok(version) = serde_json::from_value(value.clone()) {
                return Some(version);
            }
        }
        None
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
    features: Option<serde_json::Value>,
    pub num: String,
}
impl Version {
    pub fn get_features(&self) -> Option<Vec<String>> {
        if let Some(features) = &self.features {
            return Some(features.as_object().unwrap().keys().cloned().collect());
        }
        None
    }
}
