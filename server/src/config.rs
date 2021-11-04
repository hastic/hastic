use subbeat::types::{DatasourceConfig, InfluxConfig, PrometheusConfig};

pub struct Config {
    pub port: u16,
    pub datasource_config: DatasourceConfig,
}

fn resolve_datasource(config: &mut config::Config) -> anyhow::Result<DatasourceConfig> {
    if config.get::<String>("prometheus.url").is_ok() {
        return Ok(DatasourceConfig::Prometheus(PrometheusConfig {
            url: config.get("prometheus.url")?,
            query: config.get("prometheus.query")?,
        }));
    }

    if config.get::<String>("influx.url").is_ok() {
        return Ok(DatasourceConfig::Influx(InfluxConfig {
            url: config.get("influx.url")?,
            org_id: config.get("influx.org_id")?,
            token: config.get("influx.token")?,
            query: config.get("influx.query")?,
        }));
    }

    return Err(anyhow::format_err!("no datasource found"));
}

// TODO: use actual config and env variables
impl Config {
    pub fn new() -> anyhow::Result<Config> {
        let mut config = config::Config::default();

        if std::path::Path::new("config.toml").exists() {
            config.merge(config::File::with_name("config")).unwrap();
        }
        config
            .merge(config::Environment::with_prefix("HASTIC"))
            .unwrap();

        if config.get::<u16>("port").is_err() {
            config.set("port", "8000").unwrap();
        }

        Ok(Config {
            port: config.get::<u16>("port").unwrap(),
            datasource_config: resolve_datasource(&mut config)?,
        })
    }
}
