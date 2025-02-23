use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use limp::storage::{JsonDependency, JsonStorage, JsonStorageManager, PathStorage, Storage};

/// Helper function to set up a test environment.
/// Creates a temporary directory and file for testing.
/// Returns the path to the test file.
fn setup_test_env() -> PathBuf {
    let test_dir = PathBuf::from("./test_db");
    if !test_dir.exists() {
        fs::create_dir(&test_dir).unwrap(); // Create the directory if it doesn't exist
    }
    let test_file = test_dir.join("dependencies.json");
    if test_file.exists() {
        fs::remove_file(&test_file).unwrap(); // Remove the file if it already exists
    }
    test_file
}

/// Test loading and saving `JsonStorage` to a file.
#[test]
fn test_load_and_save_storage() {
    let test_file = setup_test_env();
    let storage = PathStorage::new(&test_file);

    // Create test data
    let mut dependencies = HashMap::new();
    dependencies.insert(
        "serde".to_string(),
        JsonDependency {
            name: "serde".to_string(),
            version: "1.0.0".to_string(),
            features: Some(vec!["derive".to_string()]),
            path_to_snippet: None,
        },
    );

    let json_storage = JsonStorage { dependencies };

    // Save the data to the file
    storage.save(&json_storage).unwrap();

    // Load the data back from the file
    let loaded_storage = storage.load().unwrap();

    // Verify that the data matches
    assert_eq!(loaded_storage.dependencies.len(), 1);
    assert!(loaded_storage.dependencies.contains_key("serde"));
    let serde_dep = loaded_storage.dependencies.get("serde").unwrap();

    assert_eq!(serde_dep.name, "serde");
    assert_eq!(serde_dep.version, "1.0.0");
    assert_eq!(serde_dep.features, Some(vec!["derive".to_string()]));
}

/// Test adding and removing dependencies from `JsonStorage`.
#[test]
fn test_add_and_remove_dependency() {
    let test_file = setup_test_env();
    let storage = PathStorage::new(&test_file);

    let mut json_storage = JsonStorage::default();

    // Add a dependency
    let dep = JsonDependency {
        name: "tokio".to_string(),
        version: "1.0.0".to_string(),
        features: None,
        path_to_snippet: None,
    };
    json_storage.add(dep.clone()).unwrap();

    // Save and load the storage
    storage.save(&json_storage).unwrap();
    let mut loaded_storage = storage.load().unwrap();

    // Verify that the dependency was added
    assert_eq!(loaded_storage.dependencies.len(), 1);
    assert!(loaded_storage.dependencies.contains_key("tokio"));

    // Try to add the same dependency
    assert!(loaded_storage.add(dep).is_err());

    // Remove the dependency
    json_storage.remove("tokio").unwrap();

    // Save and load the storage again
    storage.save(&json_storage).unwrap();
    let loaded_storage = storage.load().unwrap();

    // Verify that the dependency was removed
    assert_eq!(loaded_storage.dependencies.len(), 0);

    let wrong_dep = JsonDependency {
        name: "lsdkfjlksd".to_string(),
        version: "1.0.0".to_string(),
        features: None,
        path_to_snippet: None,
    };

    assert!(json_storage.add(wrong_dep).is_err());
}

/// Test updating a dependency to the latest version.
#[test]
fn test_update_dependency() {
    let test_file = setup_test_env();
    let storage = PathStorage::new(&test_file);

    let mut json_storage = JsonStorage::default();

    // Add a dependency
    let dep = JsonDependency::new("serde").unwrap();
    json_storage.add(dep.clone()).unwrap();

    // Save and load the storage
    storage.save(&json_storage).unwrap();
    let loaded_storage = storage.load().unwrap();

    // Verify that the dependency was added
    assert_eq!(loaded_storage.dependencies.len(), 1);
    assert!(loaded_storage.dependencies.contains_key("serde"));

    // Update the dependency to the latest version
    let dep = json_storage.get_mut("serde").unwrap();
    dep.update().unwrap();

    // Save and load the storage again
    storage.save(&json_storage).unwrap();
    let loaded_storage = storage.load().unwrap();

    // Verify that the version was updated
    let serde_dep = loaded_storage.dependencies.get("serde").unwrap();
    assert_ne!(serde_dep.version, "1.0.0"); // The version should have changed
}

/// Test creating a `JsonDependency` with full metadata.
#[test]
fn test_create_json_dependency_full() {
    let dep = JsonDependency::new_full("serde", Some("1.0.0"), Some(&["derive".to_string()]), None)
        .unwrap();

    // Verify that the dependency was created with the correct metadata
    assert_eq!(dep.name, "serde");
    assert_eq!(dep.version, "1.0.0");
    assert_eq!(dep.features, Some(vec!["derive".to_string()]));
}

/// Test error handling when creating a `JsonDependency`.
#[test]
fn test_create_json_dependency_errors() {
    // Test with a non-existent version
    let result = JsonDependency::new_full("serde", Some("999.999.999"), None, None);
    assert!(result.is_err()); // Expect an error because the version doesn't exist

    // Test with a non-existent snippet path
    let result = JsonDependency::new_full("serde", Some("1.0.0"), None, Some("./nonexistent/path"));
    assert!(result.is_err()); // Expect an error because the snippet path doesn't exist
}

/// Test the `Display` implementation for `JsonDependency`.
#[test]
fn test_json_dependency_display() {
    let dep = JsonDependency {
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        features: Some(vec!["derive".to_string()]),
        path_to_snippet: None,
    };

    // Verify the formatted output
    let display_output = format!("{}", dep);
    assert_eq!(
        display_output,
        r#"serde = {version = "1.0.0", features = ["derive"]}"#
    );
}

/// Test the `JsonStorageManager` functionality.
#[test]
fn test_json_storage_manager() {
    let test_file = setup_test_env();
    let storage = PathStorage::new(&test_file);
    let manager = JsonStorageManager::new(storage);

    // Create test data
    let mut dependencies = HashMap::new();
    dependencies.insert(
        "serde".to_string(),
        JsonDependency {
            name: "serde".to_string(),
            version: "1.0.0".to_string(),
            features: Some(vec!["derive".to_string()]),
            path_to_snippet: None,
        },
    );

    let json_storage = JsonStorage { dependencies };

    // Save the data using the manager
    manager.save(&json_storage).unwrap();

    // Load the data using the manager
    let loaded_storage = manager.load().unwrap();

    // Verify that the data matches
    assert_eq!(loaded_storage.dependencies.len(), 1);
    assert!(loaded_storage.dependencies.contains_key("serde"));
}
