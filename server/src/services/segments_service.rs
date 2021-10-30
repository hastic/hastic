use rusqlite::{params, Connection, Row, ToSql};

use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};

use std::iter::repeat_with;


const ID_LENGTH: usize = 20;
pub type SegmentId = String;


// TODO: make logic with this enum shorter
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum SegmentType {
    Label = 1,
    Detection = 2,
}

impl SegmentType {
    fn from(u: u64) -> SegmentType {
        if u == 1 {
            SegmentType::Label
        } else {
            SegmentType::Detection
        }
    }

    fn to_integer(&self) -> u64 {
        if *self == SegmentType::Label {
            1
        } else {
            2
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: Option<SegmentId>,
    pub from: u64,
    pub to: u64,
    pub segment_type: SegmentType,
}

impl Segment {
    fn from(row: &Row) -> anyhow::Result<Segment, rusqlite::Error> {
        Ok(Segment {
            id: row.get(0)?,
            from: row.get(1)?,
            to: row.get(2)?,
            segment_type: SegmentType::from(row.get(3)?),
        })
    }
}


pub struct SegmentsService {
    connection: Arc<Mutex<Connection>>,
}

impl SegmentsService {
    pub fn new() -> anyhow::Result<SegmentsService> {

        // TODO: move it to data service
        std::fs::create_dir_all("./data").unwrap();

        let conn = Connection::open("./data/segments.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS segment (
                      id            TEXT PRIMARY KEY,
                      start         INTEGER NOT NULL,
                      end           INTEGER NOT NULL,
                      segment_type  INTEGER NOT NULL
                 )",
            [],
        )?;

        Ok(SegmentsService {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    // returns id of created segment
    pub fn insert_segment(&self, segment: &Segment) -> anyhow::Result<Segment> {
        let id: SegmentId = repeat_with(fastrand::alphanumeric)
            .take(ID_LENGTH)
            .collect();

        // merging
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
            "INSERT INTO segment (id, start, end, segment_type) VALUES (?1, ?2, ?3, ?4)",
            params![id, min, max, segment.segment_type.to_integer()],
        )?;
        Ok(Segment {
            id: Some(id),
            from: min,
            to: max,
            segment_type: segment.segment_type,
        })
    }

    pub fn get_segments_inside(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, start, end, segment_type FROM segment WHERE ?1 < start AND end < ?2",
        )?;

        let res = stmt
            .query_map(params![from, to], Segment::from)?
            .map(|e| e.unwrap())
            .collect();
        Ok(res)
    }

    pub fn get_segments_intersected(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, start, end, segment_type
                    FROM segment
                    WHERE (start <= ?1 and ?1 <= end) OR 
                          (start <= ?2 AND ?2 <= end) OR
                          (?1 <= start AND start <= ?2) OR 
                          (?1 <= end AND end <= ?2) ",
        )?;

        let res = stmt
            .query_map(params![from, to], Segment::from)?
            .map(|e| e.unwrap())
            .collect();
        Ok(res)
    }

    pub fn delete_segments_in_range(&self, from: u64, to: u64) -> anyhow::Result<usize> {
        let conn = self.connection.lock().unwrap();
        let res = conn.execute(
            "DELETE FROM segment where ?1 <= start AND end <= ?2",
            params![from, to],
        )?;
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
