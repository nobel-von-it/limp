#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JsonDependency {
    pub name: String,

    pub version: String,

    #[serde(default)]
    pub features: Option<Vec<String>>,

    #[serde(default)]
    pub path_to_snippet: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct JsonStorage {
    #[serde(default)]
    pub dependencies: HashMap<String, JsonDependency>,
}
