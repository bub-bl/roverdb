use std::path::{Path, PathBuf};
use rover_core::constants::DEFAULT_DB_NAME;
use crate::constants::{DB_FILE_EXTENSION, DEFAULT_DB_PATH};

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        let file_name = format!("{}.{}", DEFAULT_DB_NAME, DB_FILE_EXTENSION);

        Self {
            name: DEFAULT_DB_NAME.to_string(),
            path: PathBuf::new().join(DEFAULT_DB_PATH).join(file_name),
        }
    }
}
