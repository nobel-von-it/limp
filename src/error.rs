#[derive(Debug)]
pub enum IError {
    CreateFilesError,
}

impl std::fmt::Display for IError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IError::CreateFilesError => write!(f, "Create file error"),
        }
    }
}
impl std::error::Error for IError {}
