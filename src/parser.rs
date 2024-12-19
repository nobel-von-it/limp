use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{error::LimpError, files::open};

#[derive(Debug, Clone)]
pub struct ParserEntity(String, String);

impl ParserEntity {
    pub fn from_file(path: &str) -> Result<Self, LimpError> {
        use std::io::{BufRead, BufReader};

        let file = open(path)?;
        if Path::new(path)
            .extension()
            .ok_or(LimpError::EmptyFile(path.to_string()))?
            .to_str()
            .ok_or(LimpError::EmptyFile(path.to_string()))?
            == "rs"
        {
            return Err(LimpError::NotSupported(format!("file extension: {}", path)));
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

        let imps = imports.join("\n");
        let bd = body.join("\n");

        if imps.is_empty() && bd.is_empty() {
            return Err(LimpError::EmptyFile(path.to_string()));
        }

        Ok(ParserEntity(imps, bd))
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ParserStorage {
    pub snippets: Vec<(String, bool)>,
}

impl ParserStorage {
    pub fn load() {}
}

// #[derive(Debug, Clone, Default)]
// pub struct Parser {
//     path: String,
//     imports: Option<String>,
//     body: Option<String>,
//     is_main: bool,
// }
//
// impl std::fmt::Display for Parser {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(imps) = &self.imports {
//             write!(f, "{}", imps)?
//         }
//         if let Some(bd) = &self.body {
//             write!(f, "{}", bd)?
//         }
//         write!(f, "")
//     }
// }
//
// impl Parser {
//     pub fn from_file(path: &str) -> Self {
//         use std::io::{BufRead, BufReader};
//
//         let fm = FileManager::default();
//         let file = fm.copen(path);
//         let rr = BufReader::new(file);
//
//         let mut imports = vec![];
//         let mut body = vec![];
//         let mut is_main = false;
//
//         let mut found_code = false;
//         let mut in_imp_block = false;
//
//         rr.lines().for_each(|l| {
//             if let Ok(l) = l {
//                 let tl = l.trim();
//
//                 if found_code {
//                     body.push(l.clone())
//                 } else if in_imp_block {
//                     imports.push(l.clone());
//                     if tl.ends_with("};") {
//                         in_imp_block = false;
//                     }
//                 } else if tl.starts_with("use") {
//                     imports.push(l.clone());
//                     if tl.ends_with('{') {
//                         in_imp_block = true;
//                     }
//                 } else {
//                     if l.contains("main") {
//                         is_main = true;
//                     }
//                     body.push(l.clone());
//                     found_code = true;
//                 }
//             }
//         });
//
//         Parser {
//             path: path.to_string(),
//             imports: if imports.is_empty() {
//                 None
//             } else {
//                 Some(imports.join("\n"))
//             },
//             body: if body.is_empty() {
//                 None
//             } else {
//                 Some(body.join("\n"))
//             },
//             is_main,
//         }
//     }
//     pub fn default_save(&self) -> std::io::Result<()> {
//         let fm = FileManager::default();
//         let snip_dir = fm.storage_dir();
//         println!("{}", &snip_dir);
//         println!("{}", &self.path);
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::Parser;
//
//     #[test]
//     fn default_save_test() {
//         let p = Parser::from_file("./src/parser.rs");
//         p.default_save().unwrap();
//         println!("{p}");
//     }
// }
