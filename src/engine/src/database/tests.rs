use log::info;
use env_logger;
use rover_core::constants::DEFAULT_DB_NAME;
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
