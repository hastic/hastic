pub struct Config {
    pub prom_url: String,
    pub query: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            prom_url: "http://localhost:9090".to_owned(),
            query: "rate(go_memstats_alloc_bytes_total[5m])".to_owned(),
        }
    }
}
