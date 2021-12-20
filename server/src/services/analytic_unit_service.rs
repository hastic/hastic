use std::sync::{Arc, Mutex};

use crate::utils::get_random_str;

use rusqlite::{params, Connection, Row};

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
                      last_detection  INTEGER NOT NULL
                 )",
            [],
        )?;

        Ok(AnalyticUnitService {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn resolve(&self, cfg: AnalyticUnitConfig) -> Box<dyn types::AnalyticUnit + Send + Sync> {
        match cfg {
            AnalyticUnitConfig::Threshold(c) => Box::new(ThresholdAnalyticUnit::new("1".to_string(), c.clone())),
            AnalyticUnitConfig::Pattern(c) => Box::new(PatternAnalyticUnit::new("2".to_string(), c.clone())),
            AnalyticUnitConfig::Anomaly(c) => Box::new(AnomalyAnalyticUnit::new("3".to_string(), c.clone())),
        }
    }
}