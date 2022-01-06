use std::sync::{Arc, Mutex};

use rusqlite::{Connection};


pub struct DataService {
    pub analytic_units_connection: Arc<Mutex<Connection>>,
    pub segments_connection: Arc<Mutex<Connection>>
}

impl DataService {
    pub fn new() -> anyhow::Result<DataService> {
        std::fs::create_dir_all("./data").unwrap();

        let analytic_units_connection = Connection::open("./data/analytic_units.db")?;
        let segments_connection = Connection::open("./data/segments.db")?;

        Ok(DataService {
            analytic_units_connection: Arc::new(Mutex::new(analytic_units_connection)),
            segments_connection: Arc::new(Mutex::new(segments_connection))
        })
    }
}