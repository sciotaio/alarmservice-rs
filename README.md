# Demo Alarmservice Rust
This project is the workspace root for `alarmservice`.

## Subcrates
 * `src/alarmservice-rs` - Service Main
 * `src/models-rs` - Generated DTOs

## Build (all)
```bash
cargo build --workspace
```

## Docker
In `./docker/dev/`:
```bash
docker compose up -d postgres
```
