use crate::files::FileManager;

pub struct Project {
    pub name: String,
    pub version: String,
    pub edition: String,
    // TODO: add optional fields
    pub dependencies: Vec<Dependency>,
}
impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("limp"),
            version: String::from("0.1.0"),
            edition: String::from("2021"),
            dependencies: Vec::new(),
        }
    }
}
impl Project {
    pub fn write(&self) -> std::io::Result<()> {
        use std::io::Write;

        FileManager::create_project(&self.name)?;

        let cargo_path = format!("{}/Cargo.toml", &self.name);
        let mut file = FileManager::copen(&cargo_path);

        writeln!(file, "[package]")?;
        writeln!(file, "name = \"{}\"", &self.name)?;
        writeln!(file, "version = \"{}\"", &self.version)?;
        writeln!(file, "edition = \"{}\"", &self.edition)?;
        writeln!(file)?;
        writeln!(file, "[dependencies]")?;
        if !self.dependencies.is_empty() {
            for dep in self.dependencies.iter() {
                writeln!(file, "{}", dep)?
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub features: Option<Vec<String>>,
}
impl Dependency {
    pub fn new(name: &str) -> Dependency {
        Dependency {
            name: name.to_string(),
            version: "2039482398408".to_string(),
            features: Some(vec!["hellol".to_string()]),
        }
    }
}
impl std::fmt::Display for Dependency {
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
