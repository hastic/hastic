# Hastic

Hastic needs [Prometheus](https://prometheus.io/) or [InfluxDB](https://www.influxdata.com/get-influxdb/)
instance for getting metrics.

## Build from source (Linux)

### Prerequirements
1. [Install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Install x86_64-unknown-linux-musl:  `rustup target add x86_64-unknown-linux-musl`
3. musl-tools: `sudo apt install musl-tools`


### Build
```
make
```

```
cd release
./hastic
```

open `http://localhost:4347` in browser
