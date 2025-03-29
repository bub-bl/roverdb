use crate::constants::{DB_FILE_EXTENSION, DEFAULT_DB_PATH};
use crate::database::builder::DatabaseBuilder;
use rover_core::constants::DEFAULT_DB_NAME;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        let file_name = format!("{}.{}", DEFAULT_DB_NAME, DB_FILE_EXTENSION);
        let path = PathBuf::new().join(DEFAULT_DB_PATH).join(file_name);

        DatabaseBuilder::new()
            .with_name(DEFAULT_DB_NAME)
            .with_path(&path)
            .build()
            .unwrap()
    }
}

impl Database {
    fn new(name: &str) -> Self {
        let path = PathBuf::new()
            .join(DEFAULT_DB_PATH)
            .join(format!("{}.{}", name, DB_FILE_EXTENSION));

        DatabaseBuilder::new()
            .with_name("")
            .with_path(&path)
            .build()
            .unwrap()
    }
}

impl DatabaseCreatable for Database {
    fn create(&self, name: &str) {
        let db = Database::new(name);
        let db_file = File::create(db.path);

        match db_file {
            Ok(mut f) => {
                f.write_all(b"").unwrap();
            }
            Err(e) => {
                panic!("Error creating database: {}", e)
            }
        }
    }
}

pub trait DatabaseCreatable {
    fn create(&self, name: &str);
}

pub trait DatabaseWritable {
    fn write(&self);
}

pub trait DatabaseReadable {
    fn read(&self);
}
