use std::collections::HashMap;


pub struct Config {
    pub prom_url: String,
    pub query: String,
    pub port: u16,
}

// TODO: use actual config and env variables
impl Config {
    pub fn new() -> Config {

        let mut config = config::Config::default();

        if std::path::Path::new("config.toml").exists() {
            config.merge(config::File::with_name("config")).unwrap();
        }
        config.merge(config::Environment::with_prefix("HASTIC")).unwrap();

        if config.get::<u16>("port").is_err() {
            config.set("port", "8000").unwrap();
        }
        if config.get::<String>("prom_url").is_err() {
            config.set("prom_url", "http://localhost:9090").unwrap();
        }
        if config.get::<String>("query").is_err() {
            config.set("query", "rate(go_memstats_alloc_bytes_total[5m])").unwrap();
        }

        Config {
            port: config.get::<u16>("port").unwrap(),
            prom_url: config.get("prom_url").unwrap(),
            query: config.get("query").unwrap()
        }
    }
}
