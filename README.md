# Basic ADBC Rust Client

Clone and build arrow-adbc dependency:

```bash
git clone https://github.com/apache/arrow-adbc.git
cd arrow-adbc
git checkout apache-arrow-adbc-16
rm -rf driver/snowflake/ # build errors
cargo build
```

Clone and build this repo:

```bash
git clone https://github.com/snowch/rust_adbc_client
cd rust_adbc_client
cargo build
```

Run the example:

```bash
cargo run
```
