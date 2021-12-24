use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection};

use super::analytic_service::analytic_unit::{
    anomaly_analytic_unit::AnomalyAnalyticUnit,
    pattern_analytic_unit::PatternAnalyticUnit,
    threshold_analytic_unit::ThresholdAnalyticUnit,
    types::{self, AnalyticUnitConfig},
};

#[derive(Clone)]
pub struct AnalyticUnitService {
    connection: Arc<Mutex<Connection>>,
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
                      last_detection  INTEGER,
                      active          BOOLEAN,
                      type            INTEGER,
                      config          TEXT
                 )",
            [],
        )?;

        Ok(AnalyticUnitService {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    // TODO: optional id
    pub fn resolve_au(
        &self,
        cfg: &AnalyticUnitConfig,
    ) -> Box<dyn types::AnalyticUnit + Send + Sync> {
        match cfg {
            AnalyticUnitConfig::Threshold(c) => {
                Box::new(ThresholdAnalyticUnit::new("1".to_string(), c.clone()))
            }
            AnalyticUnitConfig::Pattern(c) => {
                Box::new(PatternAnalyticUnit::new("2".to_string(), c.clone()))
            }
            AnalyticUnitConfig::Anomaly(c) => {
                Box::new(AnomalyAnalyticUnit::new("3".to_string(), c.clone()))
            }
        }
    }

    // TODO: get id of analytic_unit which be used also as it's type
    pub fn resolve(
        &self,
        cfg: &AnalyticUnitConfig,
    ) -> anyhow::Result<Box<dyn types::AnalyticUnit + Send + Sync>> {
        let au = self.resolve_au(cfg);
        let id = au.as_ref().get_id();

        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id from analytic_unit WHERE id = ?1")?;
        let res = stmt.exists(params![id])?;

        if res == false {
            let cfg_json = serde_json::to_string(&cfg)?;
            conn.execute(
                "INSERT INTO analytic_unit (id, type, config) VALUES (?1, ?1, ?2)",
                params![id, cfg_json],
            )?;
        }

        conn.execute(
            "UPDATE analytic_unit set active = FALSE where active = TRUE",
            params![],
        )?;
        conn.execute(
            "UPDATE analytic_unit set active = TRUE where id = ?1",
            params![id],
        )?;

        return Ok(au);
    }

    pub fn set_last_detection(&self, id: String, last_detection: u64) -> anyhow::Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "UPDATE analytic_unit SET last_detection = ?1 WHERE id = ?2",
            params![last_detection, id],
        )?;
        Ok(())
    }

    pub fn get_active(&self) -> anyhow::Result<Box<dyn types::AnalyticUnit + Send + Sync>> {
        // TODO: return default when there is no active
        let conn = self.connection.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id, type, config from analytic_unit WHERE active = TRUE")?;

        let au = stmt.query_row([], |row| {
            let c: String = row.get(2)?;
            let cfg: AnalyticUnitConfig = serde_json::from_str(&c).unwrap();
            Ok(self.resolve(&cfg))
        })??;

        return Ok(au);
    }

    pub fn get_active_config(&self) -> anyhow::Result<AnalyticUnitConfig> {
        let exists = {
            let conn = self.connection.lock().unwrap();
            let mut stmt = conn.prepare("SELECT config from analytic_unit WHERE active = TRUE")?;
            stmt.exists([])?
        };

        if exists == false {
            let c = AnalyticUnitConfig::Pattern(Default::default());
            self.resolve(&c)?;
            return Ok(c);
        } else {
            let conn = self.connection.lock().unwrap();
            let mut stmt = conn.prepare("SELECT config from analytic_unit WHERE active = TRUE")?;
            let acfg = stmt.query_row([], |row| {
                let c: String = row.get(0)?;
                let cfg = serde_json::from_str(&c).unwrap();
                Ok(cfg)
            })?;
            return Ok(acfg);
        }
    }

    pub fn get_config_by_id(&self, id: &String) -> anyhow::Result<AnalyticUnitConfig> {
        let exists = {
            let conn = self.connection.lock().unwrap();
            let mut stmt = conn.prepare("SELECT config from analytic_unit WHERE id = ?1")?;
            stmt.exists([id])?
        };

        if exists == false {
            let c = AnalyticUnitConfig::get_default_by_id(id);
            self.resolve(&c)?;
            return Ok(c);
        } else {
            let conn = self.connection.lock().unwrap();
            let mut stmt = conn.prepare("SELECT config from analytic_unit WHERE id = ?1")?;
            let acfg = stmt.query_row([id], |row| {
                let c: String = row.get(0)?;
                let cfg = serde_json::from_str(&c).unwrap();
                Ok(cfg)
            })?;
            return Ok(acfg);
        }
    }

    pub fn get_config_id(&self, cfg: &AnalyticUnitConfig) -> String {
        match cfg {
            AnalyticUnitConfig::Threshold(_) => "1".to_string(),
            AnalyticUnitConfig::Pattern(_) => "2".to_string(),
            AnalyticUnitConfig::Anomaly(_) => "3".to_string(),
        }
    }

    pub fn update_config_by_id(&self, id: &String, cfg: &AnalyticUnitConfig) -> anyhow::Result<()> {
        // TODO: it's possble that config doesn't exist, but we trying to update it
        let conn = self.connection.lock().unwrap();

        let cfg_json = serde_json::to_string(&cfg)?;

        conn.execute(
            "UPDATE analytic_unit SET config = ?1 WHERE id = ?2",
            params![cfg_json, id],
        )?;

        return Ok(());
    }

    pub fn update_active_config(&self, cfg: &AnalyticUnitConfig) -> anyhow::Result<()> {
        let conn = self.connection.lock().unwrap();

        let cfg_json = serde_json::to_string(&cfg)?;

        conn.execute(
            "UPDATE analytic_unit SET config = ?1 WHERE active = TRUE",
            params![cfg_json],
        )?;

        return Ok(());
    }
}
