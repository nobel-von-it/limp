use limp::crates::CratesIoDependency;
use limp::error::LimpError;

const LAST_VERSION: &str = "0.2.1";
const VERSION_LENGTH: usize = 10;
const CRATE_NAME: &str = "limp";

const FULL_CRATE_NAME: &str = "tokio";
const FEATURES: [&str; 4] = ["full", "net", "rt", "rt-multi-thread"];

/// Test fetching crate metadata from crates.io.
#[test]
fn test_from_cratesio() {
    // Fetch the crate metadata
    let dep = CratesIoDependency::from_cratesio(CRATE_NAME).unwrap();

    // Verify the response
    assert_eq!(dep.crate_info.name, CRATE_NAME);
    assert_eq!(dep.crate_info.max_version, LAST_VERSION);
    assert_eq!(dep.versions.len(), VERSION_LENGTH);
}

/// Test retrieving all versions of a crate.
#[test]
fn test_get_all_versions() {
    let crate_name = "limp";

    // Fetch the crate metadata
    let dep = CratesIoDependency::from_cratesio(crate_name).unwrap();

    // Retrieve all versions
    let all_versions = dep.get_all_versions();

    // Verify the versions
    assert_eq!(all_versions.len(), VERSION_LENGTH);
    assert_eq!(all_versions[VERSION_LENGTH - 1].num, "0.1.0");
    assert_eq!(all_versions[0].num, LAST_VERSION);
}

/// Test retrieving a specific version of a crate.
#[test]
fn test_get_version() {
    // Fetch the crate metadata
    let dep = CratesIoDependency::from_cratesio(CRATE_NAME).unwrap();

    // Retrieve a specific version
    let version = dep.get_version(0).unwrap();

    // Verify the version
    assert_eq!(version.num, LAST_VERSION);
    assert_eq!(version.crate_name, CRATE_NAME);
}

/// Test retrieving features for a specific version.
#[test]
fn test_get_features() {
    // Fetch the crate metadata
    let dep = CratesIoDependency::from_cratesio(CRATE_NAME).unwrap();

    // Retrieve features for a specific version
    assert!(dep.get_features(0).is_some_and(|f| f.is_empty()));

    let dep = CratesIoDependency::from_cratesio(FULL_CRATE_NAME).unwrap();

    // Retrieve features for a specific version
    let features = dep.get_features(0);

    assert!(features.is_some());

    let features = features.unwrap();

    // Verify the features
    assert_eq!(features.len(), 14);
    for feature in FEATURES {
        assert!(features.contains(&feature.to_string()));
    }
}

/// Test error handling for invalid version ID.
#[test]
fn test_get_version_error() {
    // Fetch the crate metadata
    let dep = CratesIoDependency::from_cratesio(CRATE_NAME).unwrap();

    // Attempt to retrieve an invalid version
    let result = dep.get_version(999);

    // Verify the error
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LimpError::VersionNotFound(_)));
}
