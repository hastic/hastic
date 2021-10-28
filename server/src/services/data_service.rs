use rusqlite::{ Connection, params };

use serde::Serialize;

use std::sync::{Arc, Mutex};


#[derive(Debug, Serialize)]
pub struct Segment {
    pub id: Option<u64>,
    pub start: u64,
    pub end: u64,
}

// TODO: find a way to remove this unsafe
unsafe impl Sync for DataService {}


pub struct DataService {
    connection: Arc<Mutex<Connection>>
}


impl DataService {
    pub fn new() -> anyhow::Result<DataService> {
        let conn = Connection::open("./data.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS segment (
                      id        INTEGER PRIMARY KEY,
                      start     INTEGER NOT NULL,
                      end       INTEGER NOT NULL
                 )",
            [],
        )?;

        Ok(DataService { connection: Arc::new(Mutex::new(conn)) })
    }

    pub fn insert_segment(&self, segment: &Segment) -> anyhow::Result<u64> {
        // TODO: merge with other segments
        self.connection.lock().unwrap().execute(
            "INSERT INTO segment (start, end) VALUES (?1, ?2)",
            params![segment.start, segment.end],
        )?;
        Ok(10)
    }

    pub fn get_segments(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, start, end FROM person WHERE ?1 < start AND end < ?2"
        )?;

        let res = stmt.query_map(params![from, to], |row| {
            Ok(Segment{ 
                id: row.get(0)?, 
                start: row.get(1)?, 
                end: row.get(2)?
            })
        })?.map(|e| e.unwrap()).collect();
        Ok(res)
    }
}
