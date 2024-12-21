use std::path::Path;

use crate::{error::LimpError, files::open, storage::JsonDependency};

#[derive(Debug, Clone)]
pub struct SnippetEntity {
    imports: Option<String>,
    body: Option<String>,
}

impl SnippetEntity {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, LimpError> {
        use std::io::{BufRead, BufReader};
        let path = path.as_ref();

        let file = open(path)?;
        if path
            .extension()
            .ok_or(LimpError::EmptyFile(path.display().to_string()))?
            .to_str()
            .ok_or(LimpError::EmptyFile(path.display().to_string()))?
            != "rs"
        {
            return Err(LimpError::NotSupported(format!(
                "file extension: {}",
                path.display()
            )));
        }

        let rr = BufReader::new(file);

        let mut imports = vec![];
        let mut body = vec![];
        let mut is_main = false;

        let mut found_code = false;
        let mut in_imp_block = false;

        rr.lines().for_each(|l| {
            if let Ok(l) = l {
                let tl = l.trim();

                if found_code {
                    body.push(l.clone())
                } else if in_imp_block {
                    imports.push(l.clone());
                    if tl.ends_with("};") {
                        in_imp_block = false;
                    }
                } else if tl.starts_with("use") {
                    imports.push(l.clone());
                    if tl.ends_with('{') {
                        in_imp_block = true;
                    }
                } else {
                    if l.contains("main") {
                        is_main = true;
                    }
                    body.push(l.clone());
                    found_code = true;
                }
            }
        });

        let imports = if imports.is_empty() {
            None
        } else {
            Some(imports.join("\n"))
        };

        let body = if body.is_empty() {
            None
        } else {
            Some(body.join("\n"))
        };

        Ok(SnippetEntity { imports, body })
    }
}
impl std::fmt::Display for SnippetEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(imp) = &self.imports {
            write!(f, "{}\n\n", imp)?;
        }
        if let Some(body) = &self.body {
            write!(f, "{}", body)
        } else {
            Ok(())
        }
    }
}

pub fn load_from_deps(deps: &[JsonDependency]) -> Option<String> {
    let mut all_imports = vec![];
    let mut all_body = vec![];
    for d in deps {
        if let Some(path) = &d.path_to_snippet {
            if let Ok(s) = SnippetEntity::from_file(path) {
                if let Some(imp) = s.imports {
                    all_imports.push(imp);
                }
                if let Some(body) = s.body {
                    all_body.push(body);
                }
            }
        }
    }

    if all_body.is_empty() {
        None
    } else {
        let imports = all_imports.join("\n");
        let body = all_body.join("\n");
        Some(format!("{}\n\n{}", imports, body))
    }
}
