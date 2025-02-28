//! # Snippet Parsing Module
//!
//! This module provides functionality for parsing and managing code snippets.
//! It includes utilities for reading Rust files, extracting imports and code blocks,
//! and combining snippets from multiple dependencies.

use std::path::Path;

use crate::{
    error::{LimpError, Result},
    files::open,
    storage::JsonDependency,
};

/// Represents a code snippet parsed from a Rust file.
///
/// This struct contains the imports and body of a Rust file, which can be used
/// to generate or combine code snippets.
#[derive(Debug, Clone)]
pub struct SnippetEntity {
    /// The imports section of the snippet (e.g., `use` statements).
    imports: Option<String>,
    /// The main body of the snippet (e.g., functions, structs, etc.).
    body: Option<String>,
}

impl SnippetEntity {
    /// Parses a Rust file into a `SnippetEntity`.
    ///
    /// This function reads a Rust file, extracts the imports and code blocks,
    /// and returns a `SnippetEntity` containing the parsed data.
    ///
    /// # Arguments
    /// * `path` - The path to the Rust file.
    ///
    /// # Returns
    /// - `Ok(SnippetEntity)` if the file is successfully parsed.
    /// - `Err(LimpError)` if the file is not a Rust file or cannot be read.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        use std::io::{BufRead, BufReader};
        let path = path.as_ref();

        let file = open(path)?;
        // Check if the file is a rust file and if not return an error
        // Two wrappers are used but I want to change it
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
    /// Formats the `SnippetEntity` for display.
    ///
    /// This function formats the `SnippetEntity` as a string, with imports (if any)
    /// followed by the body of the snippet.
    /// If it's provided multiple snippets it won't combine them.
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

/// Combines snippets from multiple dependencies into a single string.
///
/// This function takes a list of dependencies, extracts their associated snippets,
/// and combines the imports and bodies into a single string.
///
/// # Arguments
/// * `deps` - A slice of `JsonDependency` objects.
///
/// # Returns
/// - `Some(String)` if snippets are successfully combined.
/// - `None` if no snippets are found.
pub fn load_from_deps(deps: &[JsonDependency]) -> Option<String> {
    let mut all_imports = vec![];
    let mut all_body = vec![];
    for d in deps {
        if let Some(path) = &d.path_to_snippet {
            // Base code, will be replaced after big research
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
