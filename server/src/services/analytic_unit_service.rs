use crate::utils::get_random_str;

use rusqlite::{params, Connection, Row};

pub struct AnalyticUnitService {
    // TODO: resolve by setting id for 3 types 
    // TODO: create database
    // TODO: update detection

    
}

impl AnalyticUnitService {
    pub fn new() -> anyhow::Result<AnalyticUnitService> {
        // TODO: remove repetitoin with segment_service
        std::fs::create_dir_all("./data").unwrap();
        let conn = Connection::open("./data/analytic_units.db")?;

        // TODO: add learning results field
        conn.execute(
            "CREATE TABLE IF NOT EXISTS analytic_unit (
                      id              TEXT PRIMARY KEY,
                      last_detection  INTEGER NOT NULL,
                 )",
            [],
        )?;

        Ok(AnalyticUnitService {
            connection: Arc::new(Mutex::new(conn)),
        })
    }
}