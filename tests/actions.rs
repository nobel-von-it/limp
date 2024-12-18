use actions::{Action, CommandHandler};
use error::LimpError;

use crate::files::{self, open};
use crate::storage::{JsonDependency, JsonStorage};
use limp::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Mock for JsonStorage
#[derive(Default)]
struct MockJsonStorage {
    dependencies: HashMap<String, JsonDependency>,
}

impl MockJsonStorage {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, LimpError> {
        // Return a mock storage
        Ok(MockJsonStorage::default())
    }

    fn add(&mut self, dependency: JsonDependency) {
        self.dependencies
            .insert(dependency.name.clone(), dependency);
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), LimpError> {
        // Mock save to file (do nothing)
        Ok(())
    }

    fn remove(&mut self, name: &str) {
        self.dependencies.remove(name);
    }
}

// Mock function for creating projects
fn mock_create_project(name: &str, dependencies: Option<&[String]>) -> Result<(), LimpError> {
    Ok(()) // No-op for testing
}

// Test for `CommandHandler::build` method
#[test]
fn test_command_handler_build() {
    let command = CommandHandler::build();
    let app_name = command.get_name();
    assert_eq!(app_name, "limp");
}

// Test for parsing `init` action from CLI args
#[test]
fn test_command_handler_parse_init() {
    let args = vec![
        "limp",
        "init",
        "my_project",
        "--dependencies",
        "dep1",
        "dep2",
    ];
    let matches = CommandHandler::build().get_matches_from(args);
    let handler = CommandHandler::parse(&matches);

    if let Some(Action::Init { name, dependencies }) = handler.action {
        assert_eq!(name, "my_project");
        assert_eq!(
            dependencies.unwrap(),
            vec!["dep1".to_string(), "dep2".to_string()]
        );
    } else {
        panic!("Failed to parse init action");
    }
}

// Test for parsing `new` action from CLI args
#[test]
fn test_command_handler_parse_new_dependency() {
    let args = vec!["limp", "new", "dep_name", "--version", "1.0.0"];
    let matches = CommandHandler::build().get_matches_from(args);
    let handler = CommandHandler::parse(&matches);

    if let Some(Action::NewDependency { name, version, .. }) = handler.action {
        assert_eq!(name, "dep_name");
        assert_eq!(version, Some("1.0.0".to_string()));
    } else {
        panic!("Failed to parse new dependency action");
    }
}

// Test for the `make_action` method (init action)
#[test]
fn test_make_action_init() {
    let handler_err = CommandHandler {
        action: Some(Action::Init {
            name: "my_project".to_string(),
            dependencies: Some(vec!["dep1".to_string(), "dep2".to_string()]),
        }),
    };

    // Mock the `create_project` function
    assert!(handler_err.make_action().is_err());

    let handler_ok = CommandHandler {
        action: Some(Action::Init {
            name: "my_project".to_string(),
            dependencies: None,
        }),
    };

    assert!(handler_ok.make_action().is_ok());

    fs::remove_dir_all("my_project").unwrap();
}

// Test for the `make_action` method (new dependency action)
#[test]
fn test_make_action_new_dependency() {
    let handler = CommandHandler {
        action: Some(Action::NewDependency {
            name: "non_existing_dep".to_string(),
            version: Some("1.0.0".to_string()),
            features: None,
            path_to_snippet: None,
        }),
    };

    // Mock the actions and test
    assert!(handler.make_action().is_err());

    let handler = CommandHandler {
        action: Some(Action::NewDependency {
            name: "tokio".to_string(),
            version: Some("1.0.0".to_string()),
            features: None,
            path_to_snippet: None,
        }),
    };

    // Mock the actions and test
    assert!(handler.make_action().is_ok());
}

// Test for `make_action` (list dependencies action)
#[test]
fn test_make_action_list() {
    let handler = CommandHandler {
        action: Some(Action::List),
    };

    // Mock the behavior of the list action
    handler
        .make_action()
        .expect("Expected list action to succeed");
}

// Test for `make_action` (delete dependency action)
#[test]
fn test_make_action_delete() {
    let handler = CommandHandler {
        action: Some(Action::Delete {
            name: "dep_to_delete".to_string(),
        }),
    };

    // Mock delete logic
    handler
        .make_action()
        .expect("Expected delete action to succeed");
}

// Test for `make_action` (add dependency to project action)
#[test]
fn test_make_action_add() {
    let handler_err = CommandHandler {
        action: Some(Action::Add {
            name: "dep_to_add".to_string(),
        }),
    };

    // Mock the add dependency logic
    assert!(handler_err.make_action().is_err());

    let handler_create = CommandHandler {
        action: Some(Action::Init {
            name: "my_project_add".to_string(),
            dependencies: None,
        }),
    };

    assert!(handler_create.make_action().is_ok());

    let handler_ok = CommandHandler {
        action: Some(Action::Add {
            name: "tokio".to_string(),
        }),
    };

    assert!(handler_ok.make_action().is_ok());

    fs::remove_dir_all("my_project_add").unwrap();
}
