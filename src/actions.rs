use crate::{
    crates::FullCrateInfo,
    json::{self, DependencyInfo},
    toml::{Dependency, Project},
};

pub enum Action {
    Init {
        name: String,
        dependencies: Option<Vec<String>>,
    },
    NewDependency {
        name: String,
        version: Option<String>,
        features: Option<Vec<String>>,
        path_to_snippet: Option<String>,
    },
    Delete {
        name: String,
    },
    List,
    Version,
    Help,
}
impl Action {
    pub fn init(name: &str, mbdeps: &Option<Vec<String>>) {
        // loading hashmap with all dependencies read only
        let jd = json::load();

        // mini debug
        println!("Initialize project with name {}", &name);

        // Initialize variable for project
        let mut resdeps = vec![];

        // figure out dependencies
        if let Some(deps) = mbdeps {
            for d in deps.iter() {
                // check existing
                match json::get_dependency(&jd, d) {
                    Some(d) => resdeps.push(d),
                    None => {
                        // get default last version from cratesio if not exist
                        if let Some(new_dep) = FullCrateInfo::get_from_cratesio(d) {
                            resdeps.push(Dependency::from(new_dep));
                        }
                    }
                }
            }
        }

        // create project structure
        let project = Project::new(name, resdeps);

        // write all information into Cargo.toml
        project.write().map_err(|e| eprintln!("{e}")).unwrap()
    }
    pub fn add_new(
        name: &str,
        version: &Option<String>,
        features: &Option<Vec<String>>,
        path_to_snippet: &Option<String>,
    ) {
        // loading hashmap with all dependencies read-append
        let mut jd = json::load();

        // check name in hashmap and
        if !jd.iter().any(|(n, _)| n == name) {
            // get info about crate from cratesio
            if let Some(crate_) = FullCrateInfo::get_from_cratesio(name) {
                // initialize base dependency info
                let mut dep_info = DependencyInfo::default();

                if let Some(path_to_snippet) = path_to_snippet {
                    if std::path::Path::new(path_to_snippet).exists() {
                        dep_info.path_to_snippet = Some(path_to_snippet.to_string());
                    } else {
                        println!("{path_to_snippet} not exist");
                    }
                }
                // check version provided and version is valid else get latest version from cratesio
                let res_version = if let Some(ver) = version {
                    if let Some(version) = crate_.get_all_versions().iter().find(|v| &v.num == ver)
                    {
                        version.clone()
                    } else {
                        crate_.get_version(0).unwrap()
                    }
                } else {
                    crate_.get_version(0).unwrap()
                };
                dep_info.version = res_version.num.clone();

                // check features provided
                if let Some(features) = features {
                    let mut res_features = vec![];
                    for f in features.iter() {
                        if let Some(d_features) = res_version.get_features() {
                            // mini debug
                            // println!("{}", f);
                            // println!("{:?}", d_features.clone());

                            // check feature valid
                            if d_features.contains(f) {
                                res_features.push(f.to_string());
                            } else {
                                eprintln!("feature {f} doesn't exist");
                            }
                        }
                    }
                    dep_info.features = Some(res_features);
                }

                // insert can't return some
                let _ = jd.insert(name.to_string(), dep_info);
                json::save(&jd);
            } else {
                eprintln!("crate {name} not in cratesio");
            }
        } else {
            // TODO: rewrite crate question
            eprintln!("crate with name {name} exist");
        }

        // if let Some(d) = jd.insert(
        //     name.to_string(),
        //     DependencyInfo {
        //         version: version.to_string(),
        //         features,
        //         path_to_snippet,
        //     },
        // ) {
        //     Some(Dependency {
        //         name: name.to_string(),
        //         version: d.version.clone(),
        //         features: d.features.clone(),
        //     })
        // } else {
        //     None
        // }
    }
    pub fn delete(name: &str) {
        let mut jd = json::load();
        if jd.iter().any(|(n, _)| n == name) {
            jd.remove(name);
            println!("{name} deleted")
        } else {
            println!("{name} doesn't exist");
        }
        json::save(&jd);
    }
    pub fn list() {
        let jd = json::load();
        for (k, v) in jd.iter() {
            println!("{k} ->");
            println!("  v: {ver}", ver = &v.version);
            if let Some(features) = &v.features {
                println!("  f: {fs}", fs = &features.join(", "))
            }
            if let Some(path) = &v.path_to_snippet {
                println!("  p: {path}")
            }
        }
    }
    pub fn version() {
        println!("version: 0.1.4")
    }
}
