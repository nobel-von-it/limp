pub fn parse_dependency_version(version: String) -> String {
    match version.split('.').count() {
        3 => version,
        2 => format!("{}.0", version),
        1 => format!("{}.0.0", version),
        _ => unreachable!(),
    }
}
