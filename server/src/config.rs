use subbeat::types::{DatasourceConfig, InfluxConfig, PrometheusConfig};

use std::env;

#[derive(Clone)]
pub struct WebhookAlertingConfig {
    pub endpoint: String,
}

#[derive(Clone)]
pub enum AlertingType {
    Webhook(WebhookAlertingConfig),
}

#[derive(Clone)]
pub struct AlertingConfig {
    pub alerting_type: AlertingType,
    pub interval: u64, // interval in seconds
}

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub datasource_config: DatasourceConfig,
    pub alerting: Option<AlertingConfig>,
}

fn resolve_datasource(config: &config::Config) -> anyhow::Result<DatasourceConfig> {
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

fn resolve_alerting(config: &config::Config) -> anyhow::Result<Option<AlertingConfig>> {
    if config.get::<String>("alerting.type").is_err() {
        return Ok(None);
    }

    if config.get::<String>("alerting.endpoint").is_err() {
        return Err(anyhow::format_err!("missing endpoint param in alerting"));
    }
    if config.get::<String>("alerting.interval").is_err() {
        return Err(anyhow::format_err!("missing interval param in alerting"));
    }
    if config.get::<u64>("alerting.interval").is_err() {
        return Err(anyhow::format_err!(
            "alerting interval should be a positive integer number"
        ));
    }
    let analytic_type = config.get::<String>("alerting.type").unwrap();
    if analytic_type != "webhook" {
        return Err(anyhow::format_err!(
            "unknown alerting type: {}",
            analytic_type
        ));
    }

    let endpoint = config.get::<String>("alerting.endpoint").unwrap();
    let interval = config.get::<u64>("alerting.interval").unwrap();
    return Ok(Some(AlertingConfig {
        alerting_type: AlertingType::Webhook(WebhookAlertingConfig { endpoint }),
        interval,
    }));
}

// config::Environment doesn't support nested configs, e.g. `alerting.type`,
// so I've copied this from:
// https://github.com/rust-lang/mdBook/blob/f3e5fce6bf5e290c713f4015947dc0f0ad172d20/src/config.rs#L132
// so that `__` can be used in env variables instead of `.`, 
// e.g. `HASTIC_ALERTING__TYPE` -> alerting.type
pub fn update_from_env(config: &mut config::Config) {
    let overrides =
        env::vars().filter_map(|(key, value)| parse_env(&key).map(|index| (index, value)));

    for (key, value) in overrides {
        println!("{} => {}", key, value);
        config.set(&key, value).unwrap();
    }
}

fn parse_env(key: &str) -> Option<String> {
    const PREFIX: &str = "HASTIC_";

    if key.starts_with(PREFIX) {
        let key = &key[PREFIX.len()..];
        Some(key.to_lowercase().replace("__", ".").replace("_", "-"))
    } else {
        None
    }
}

impl Config {
    pub fn new() -> anyhow::Result<Config> {
        let mut config = config::Config::default();

        if std::path::Path::new("config.toml").exists() {
            config.merge(config::File::with_name("config")).unwrap();
        }

        update_from_env(&mut config);

        if config.get::<u16>("port").is_err() {
            config.set("port", "8000").unwrap();
        }

        // TODO: print resulted config (perfectly, it needs adding `derive(Debug)` in `subbeat`'s `DatasourceConfig`)
        Ok(Config {
            port: config.get::<u16>("port").unwrap(),
            datasource_config: resolve_datasource(&config)?,
            alerting: resolve_alerting(&config)?,
        })
    }
}
