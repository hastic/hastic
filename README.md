# Hastic

Hastic needs [Prometheus](https://prometheus.io/) or [InfluxDB](https://www.influxdata.com/get-influxdb/)
instance for getting metrics.

## Build from source (Linux)

### Prerequirements
1. [Install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (required version: >=1.49)
2. Install [node.js >=10.x](https://nodejs.org/en/download/)
3. Install [yarn](https://classic.yarnpkg.com/lang/en/docs/install)
4. Install x86_64-unknown-linux-musl:  `rustup target add x86_64-unknown-linux-musl`
5. musl-tools: `sudo apt install musl-tools`

### Build
```
make
```

```
cd release
./hastic
```

open `http://localhost:4347` in browser
