use crate::files::FileManager;

#[derive(Debug, Clone, Default)]
pub struct Parser {
    imports: Option<String>,
    body: Option<String>,
    is_main: bool,
}

impl Parser {
    pub fn from_file(path: &str) -> Self {
        use std::io::{BufRead, BufReader};

        let fm = FileManager::default();
        let file = fm.copen(path);
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

        Parser {
            imports: if imports.is_empty() {
                None
            } else {
                Some(imports.join("\n"))
            },
            body: if body.is_empty() {
                None
            } else {
                Some(body.join("\n"))
            },
            is_main,
        }
    }
    pub fn default_save(&self, path: &str) -> std::io::Result<()> {
        Ok(())
    }
}
