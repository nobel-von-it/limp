use serde::{Deserialize, Serialize};

use crate::{files::FileManager, toml::Dependency};

const CONFIG_PATH: &str = "/home/nerd/.config/limp/dependencies.json";

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DependencyInfo {
    pub version: String,
    pub features: Option<Vec<String>>,
    pub path_to_snippet: Option<String>,
}
pub type JsonDependencies = std::collections::HashMap<String, DependencyInfo>;

pub fn load() -> JsonDependencies {
    let file = FileManager::copen(CONFIG_PATH);
    serde_json::from_reader(file).unwrap_or(JsonDependencies::new())
}

pub fn save(jd: &JsonDependencies) {
    let file = FileManager::copen(CONFIG_PATH);
    file.set_len(0).unwrap();
    serde_json::to_writer(file, jd).unwrap_or_else(|e| {
        eprintln!("ERROR: serde_json error: {e}");
        std::process::exit(1);
    })
}

pub fn get_dependency(jd: &JsonDependencies, name: &str) -> Option<Dependency> {
    jd.get(name).map(|dep| Dependency {
        name: name.to_string(),
        version: dep.version.clone(),
        features: dep.features.clone(),
    })
}

pub fn get_dependency_info(jd: &JsonDependencies, name: &str) -> Option<DependencyInfo> {
    jd.get(name).cloned()
}

pub fn get_path(jd: &JsonDependencies, name: &str) -> Option<String> {
    if let Some(dep) = jd.get(name) {
        dep.path_to_snippet.as_ref().map(|p| p.to_string())
    } else {
        None
    }
}
