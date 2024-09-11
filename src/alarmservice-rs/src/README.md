# Demo Alarmservice rust

## Build
```bash
cargo build
```

## Run
Run with default profile
```bash
cargo run
```
Run with custom profile
```bash
cargo run -- --config config/custom.yml
```


## Docker
In `./docker/dev/`:
```bash
docker compose up -d postgres
```
