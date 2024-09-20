use serde::{Deserialize, Serialize};

use crate::toml::Dependency;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullCrateInfo {
    #[serde(rename = "crate")]
    pub crate_info: Crate,
    versions: Vec<serde_json::Value>,
}
impl FullCrateInfo {
    pub fn get_from_cratesio(name: &str) -> Option<Self> {
        let url = format!("https://crates.io/api/v1/crates/{}", name);
        if let Ok(res) = reqwest::blocking::Client::new()
            .get(url)
            .header("User-Agent", "limp/0.1.0")
            .send()
        {
            if let Ok(text) = res.json::<FullCrateInfo>() {
                return Some(text);
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
    name: String,
    max_version: String,
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

#[derive(Debug)]
pub struct CrateValidator {
    pub name: String,
    pub versions: Vec<String>,
    pub features: Option<Vec<String>>,
}

impl CrateValidator {
    pub fn get_last_from_cratesio(name: &str) -> Option<Self> {
        let url = format!("https://crates.io/api/v1/crates/{}", name);
        if let Ok(res) = reqwest::blocking::Client::new()
            .get(url)
            .header("User-Agent", "limp/0.1.0")
            .send()
        {
            if let Ok(text) = res.json::<FullCrateInfo>() {
                let versions = text.get_all_versions();
                return Some(CrateValidator {
                    name: text.crate_info.name.to_string(),
                    versions: versions.iter().map(|v| v.num.to_string()).collect(),
                    features: versions[0].get_features(),
                });
            }
        }
        None
    }
    pub fn validate(name: &str) -> bool {
        CrateValidator::get_last_from_cratesio(name).is_some()
    }
    pub fn dependency_validate(d: &Dependency) -> bool {
        if let Some(cv) = CrateValidator::get_last_from_cratesio(&d.name) {
            println!("{:#?}", &cv);
            println!("{:#?}", d);
            if cv.name == d.name && cv.versions.contains(&d.version) {
                if d.features.is_none() {
                    return true;
                } else if cv.features.is_some() && d.features.is_some() {
                    let cvf = cv.features.clone().unwrap();
                    let df = d.features.clone().unwrap();
                    if df
                        .iter()
                        .filter(|f| cvf.contains(f))
                        .collect::<Vec<_>>()
                        .len()
                        == df.len()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}
