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

### Configure
Hastic can be configured using config-file or environment variables.

At first, choose which datasource you'll be using: `prometheus` or `influx`. Only one can be used at a time.

#### Config-file
- copy the config example to the release directory
```bash
cp config.example.toml release/config.toml
```
- edit the config file, e.g. using `nano`
```bash
nano release/config.toml
```

#### Environment variables
All config fields are also available as environment variables with `HASTIC_` prefix

Variable name structure:
- for high-level fields: `HASTIC_<field_name>`, e.g. `HASTIC_PORT`
- for nested fields: `HASTIC_<category_name>__<field_name>`, e.g. `HASTIC_PROMETHEUS__URL`

Environment variables can be set either by exporting them (they'll be actual until a bash-session is closed):
```bash
export HASTIC_PORT=8000
export HASTIC_PROMETHEUS__URL=http://localhost:9090
export HASTIC_PROMETHEUS__QUERY=rate(go_memstats_alloc_bytes_total[5m])
```

or specifing them in a run command (they'll be actual only for one run)
```bash
HASTIC_PORT=8000 HASTIC_PROMETHEUS__URL=http://localhost:9090 HASTIC_PROMETHEUS__QUERY=rate(go_memstats_alloc_bytes_total[5m]) ./release/hastic
```

### Run

```
cd release
./hastic
```

open `http://localhost:4347` in browser
