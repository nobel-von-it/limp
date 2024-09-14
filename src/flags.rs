use crate::toml::{Dependency, Project};

pub enum Action {
    Init { name: String },
    HelloWorld,
}

#[derive(Default)]
pub struct Config {
    pub action: Option<Action>,
}
impl Config {
    pub fn parse(args: &[String]) -> Self {
        let mut config = Config::default();
        let mut action_exists = false;
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "init" => {
                    if !action_exists {
                        if let Some(name) = args.next() {
                            config.action = Some(Action::Init {
                                name: name.to_string(),
                            })
                        }
                        action_exists = true;
                    }
                }
                "hello" | "helloworld" | "hw" => {
                    if !action_exists {
                        config.action = Some(Action::HelloWorld);
                        action_exists = true;
                    }
                }
                _ => {}
            }
        }
        config
    }
    pub fn make_action(&self) {
        if let Some(act) = &self.action {
            match act {
                Action::Init { name } => {
                    println!("Initialize project with name {name}");
                    let proj = Project {
                        name: name.to_string(),
                        version: "0.1.0".to_string(),
                        edition: "2021".to_string(),
                        dependencies: vec![Dependency::new("hello")],
                    };
                    proj.write().map_err(|e| eprintln!("{e}")).unwrap()
                }
                Action::HelloWorld => println!("Hello world from command"),
            }
        }
    }
}
