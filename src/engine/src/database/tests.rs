use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use log::{info, warn};
    use serde::{Deserialize, Serialize};
    use rover_core::constants::DEFAULT_DB_NAME;
    use crate::database::binary::{BinaryFile, BinaryWriter};
    use crate::database::database::Database;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_default_db_path_dist() {
        init();

        let db = Database::default();
        let db_path = format!("db\\{}.bin", DEFAULT_DB_NAME);

        info!("Path: {}", db_path);
        info!("Database Path: {}", db.path.to_str().unwrap());

        assert_eq!(db_path, db.path.to_str().unwrap());
    }

    #[test]
    fn test_write_db_file() {
        init();

        let db = Database::default();
    }

    #[test]
    fn test_write_binary_file() {
        init();

        let mut binary = BinaryWriter::new();
        let path = PathBuf::new().join("test_bin/test_doc.bin");
        let document = Document {
            name: "A document.".to_string(),
        };
        binary.write(path, document).unwrap();
    }

    #[test]
    fn test_read_binary_file() {
        init();

        let binary = BinaryWriter::new();
        let path = PathBuf::new().join("test_bin/test_doc.bin");
        let document = Document {
            name: "A document.".to_string(),
        };
        let decoded_doc: BinaryFile<Document> = binary.read(path).unwrap();
        warn!("Document: {:?}", decoded_doc);
        // assert_eq!(document, decoded_doc);
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Document {
        name: String
    }
}