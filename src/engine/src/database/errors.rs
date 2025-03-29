use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseBuilderError {
    #[error("Missing name")]
    MissingName,
    #[error("Missing path")]
    MissingPath,
}