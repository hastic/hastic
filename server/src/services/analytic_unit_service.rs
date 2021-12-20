use std::sync::{Arc, Mutex};

use crate::utils::get_random_str;

use rusqlite::{params, Connection, Row};
use warp::hyper::rt::Executor;

use super::analytic_service::analytic_unit::{types::{AnalyticUnitConfig, self}, threshold_analytic_unit::ThresholdAnalyticUnit, pattern_analytic_unit::PatternAnalyticUnit, anomaly_analytic_unit::AnomalyAnalyticUnit};

#[derive(Clone)]
pub struct AnalyticUnitService {
    // TODO: resolve by setting id for 3 types 
    // TODO: create database
    // TODO: update detection
    connection: Arc<Mutex<Connection>>
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
                      last_detection  INTEGER
                 )",
            [],
        )?;

        Ok(AnalyticUnitService {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    // TODO: optional id
    pub fn resolve_au(&self, cfg: AnalyticUnitConfig) -> Box<dyn types::AnalyticUnit + Send + Sync> {
        match cfg {
            AnalyticUnitConfig::Threshold(c) => Box::new(ThresholdAnalyticUnit::new("1".to_string(), c.clone())),
            AnalyticUnitConfig::Pattern(c) => Box::new(PatternAnalyticUnit::new("2".to_string(), c.clone())),
            AnalyticUnitConfig::Anomaly(c) => Box::new(AnomalyAnalyticUnit::new("3".to_string(), c.clone())),
        }
    }

    pub fn resolve(&self, cfg: AnalyticUnitConfig) -> anyhow::Result<Box<dyn types::AnalyticUnit + Send + Sync>> {
        let au = self.resolve_au(cfg);
        let id = au.as_ref().get_id();

        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id from analytic_unit WHERE id = ?1",
        )?;
        let res = stmt.exists(params![id])?;

        if res == false {
            conn.execute(
            "INSERT INTO analytic_unit (id) VALUES (?1)",
            params![id]
            )?;
        }

        return Ok(au);
    }

    // TODO: resolve with saving by id
    pub fn set_last_detection(&self, id: String, last_detection: u64) -> anyhow::Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "UPDATE analytic_unit SET last_detection = ?1 WHERE id = ?2",
            params![last_detection, id]
        )?;
        Ok(())
    }
}