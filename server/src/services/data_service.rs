use rusqlite::{params, Connection};

use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};

use std::iter::repeat_with;

const ID_LENGTH: usize = 20;
pub type SegmentId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: Option<SegmentId>,
    pub from: u64,
    pub to: u64,
}

// TODO: find a way to remove this unsafe
unsafe impl Sync for DataService {}

pub struct DataService {
    connection: Arc<Mutex<Connection>>,
}

impl DataService {
    pub fn new() -> anyhow::Result<DataService> {
        let conn = Connection::open("./data.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS segment (
                      id        TEXT PRIMARY KEY,
                      start     INTEGER NOT NULL,
                      end       INTEGER NOT NULL
                 )",
            [],
        )?;

        Ok(DataService {
            connection: Arc::new(Mutex::new(conn))
        })
    }

    // returns id of created segment
    pub fn insert_segment(&self, segment: &Segment) -> anyhow::Result<SegmentId> {
        let id: SegmentId = repeat_with(fastrand::alphanumeric)
            .take(ID_LENGTH)
            .collect();
        // TODO: merge with other segments
        self.connection.lock().unwrap().execute(
            "INSERT INTO segment (id, start, end) VALUES (?1, ?2, ?3)",
            params![id, segment.from, segment.to],
        )?;
        Ok(id)
    }

    pub fn get_segments(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id, start, end FROM segment WHERE ?1 < start AND end < ?2")?;

        let res = stmt
            .query_map(params![from, to], |row| {
                Ok(Segment {
                    id: row.get(0)?,
                    from: row.get(1)?,
                    to: row.get(2)?,
                })
            })?
            .map(|e| e.unwrap())
            .collect();
        Ok(res)
    }
}
