use rusqlite::{params, Connection, ToSql};

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
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    // returns id of created segment
    pub fn insert_segment(&self, segment: &Segment) -> anyhow::Result<Segment> {
        let id: SegmentId = repeat_with(fastrand::alphanumeric)
            .take(ID_LENGTH)
            .collect();

        // merging
        // TODO extract intersected ids
        // TODO: merge with other segments
        let sgms = self.get_segments_intersected(segment.from, segment.to)?;
        let mut min = segment.from;
        let mut max = segment.to;
        let mut ids_to_delete = Vec::<SegmentId>::new();
        for s in sgms {
            min = min.min(s.from);
            max = max.max(s.to);
            ids_to_delete.push(s.id.unwrap());
        }
        self.delete_segments(&ids_to_delete)?;

        self.connection.lock().unwrap().execute(
            "INSERT INTO segment (id, start, end) VALUES (?1, ?2, ?3)",
            params![id, min, max],
        )?;
        Ok(Segment {
            id: Some(id),
            from: min,
            to: max,
        })
    }

    pub fn get_segments_inside(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
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

    pub fn get_segments_intersected(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, start, end FROM segment 
                                                  WHERE (start <= ?1 and ?1 <= end) OR 
                                                        (start <= ?2 AND ?2 <= end) OR
                                                        (?1 <= start AND start <= ?2) OR 
                                                        (?1 <= end AND end <= ?2) ",
        )?;

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

    pub fn delete_segments(&self, ids: &Vec<SegmentId>) -> anyhow::Result<usize> {
        if ids.len() == 0 {
            return Ok(0);
        }
        // TODO: here could be sql injection if you substitute id with string :)
        let conn = self.connection.lock().unwrap();
        let ids_comma = ids
            .iter()
            .map(|id| "\"".to_string() + id + "\"")
            .collect::<Vec<String>>()
            .join(",");
        let query_str = format!("DELETE FROM segment WHERE id in ({})", ids_comma);
        let mut stmt = conn.prepare(&query_str)?;
        let res = stmt.execute([])?;
        Ok(res)
    }
}
