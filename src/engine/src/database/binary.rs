use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::error::Error;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use sha2::Digest;

#[derive(Debug, Serialize, Deserialize)]
pub enum BinaryFileVersion {
    V1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryFile<T>
    where
        T: Serialize,
{
    pub header: BinaryHeader,
    pub document: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryHeader {
    pub name: String,
    pub version: BinaryFileVersion,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug)]
pub enum BinaryWriterError {
    FailedToWriteFile,
    FailedToSetPermissions,
    FailedToCreateFile,
}

#[derive(Debug)]
pub enum DirectoryType {
    Cache,
    Store,
    Log,
    Manifest,
    Temp,
}

impl DirectoryType {
    pub fn path(&self) -> &'static str {
        match self {
            DirectoryType::Cache => "cache",
            DirectoryType::Store => "store",
            DirectoryType::Log => "log",
            DirectoryType::Manifest => "manifest",
            DirectoryType::Temp => "temp",
        }
    }
}

#[derive(Debug)]
pub struct BinaryWriter {}

impl BinaryWriter {
    pub fn new() -> Self {
        Self {}
    }

    fn is_binary_file(path: &Path) -> bool {
        path.is_file() && path.extension().unwrap() == "bin"
    }

    pub fn write<T: Serialize>(&mut self, path: PathBuf, data: T) -> Result<(), BinaryWriterError> {
        let binary_file = BinaryFile {
            header: BinaryHeader {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                version: BinaryFileVersion::V1,
                created: Utc::now(),
                last_updated: Utc::now(),
            },
            document: data,
        };

        let encoded_data = bincode::serialize(&binary_file).unwrap();
        let mut file = File::create(path).unwrap();

        file.write_all(&encoded_data)
            .map_err(|_| BinaryWriterError::FailedToWriteFile)?;

        Ok(())
    }

    pub fn read<T: Serialize + DeserializeOwned>(&self, path: PathBuf) -> Result<BinaryFile<T>, Box<dyn Error>> {
        if !Self::is_binary_file(&path) {
            return Err("Not a binary file".into());
        }

        let mut file = File::open(&path)?;

        let mut encoded_data = Vec::new();
        file.read_to_end(&mut encoded_data)?;

        let decoded_data = bincode::deserialize(&encoded_data)?;
        Ok(decoded_data)
    }
}