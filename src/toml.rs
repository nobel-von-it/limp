const CARGO_PATH: &str = "Test.toml";

pub struct Project {
    pub name: String,
    pub version: String,
    pub edition: String,
    // TODO: add optional fields
    pub dependencies: Vec<Dependency>,
}
impl Project {
    pub fn write(&self) -> std::io::Result<()> {
        use std::io::Write;

        let mut file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(CARGO_PATH)
            .unwrap_or_else(|_| std::fs::File::create(CARGO_PATH).unwrap());

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

pub struct Dependency {
    name: String,
    version: String,
    features: Option<Vec<String>>,
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
