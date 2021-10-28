pub struct Config {
    pub prom_url: String,
    pub query: String,
    pub port: u16
}

// TODO: use actual config and env variables
impl Config {
    pub fn new() -> Config {
        Config {
            port: 8000,
            prom_url: "http://localhost:9090".to_owned(),
            query: "rate(go_memstats_alloc_bytes_total[5m])".to_owned(),
        }
    }
}
