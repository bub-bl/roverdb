use crate::database::database::Database;
use crate::database::errors::DatabaseBuilderError;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DatabaseBuilder {
    name: Option<String>,
    path: Option<PathBuf>,
}

impl DatabaseBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            path: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.path = Some(path.to_path_buf());
        self
    }

    pub fn build(self) -> Result<Database, DatabaseBuilderError> {
        let name = self.name.ok_or("Database name is required").unwrap();
        let path = self.path.ok_or("Database path is required").unwrap();

        Ok(Database { name, path })
    }
}
