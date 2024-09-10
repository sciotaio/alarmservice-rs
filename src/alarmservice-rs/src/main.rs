mod config;
mod tracing;
mod persistence;

use std::{thread, time::Duration};

use config::AppConfig;
use persistence::database::init_connection;
use ::tracing::info;
use tracing::init_tracing;

#[tokio::main]
async fn main() {
    let config = AppConfig::parse().expect("Parsing config failed!");

    // init tracing
    let directives = &config.logging.levels.join(",");
    init_tracing(directives);

    // init DB
    init_connection(config.postgres).await;

    info!("Initialization done.");
    info!("Starting server at port {} ...", config.server.port);

    thread::sleep(Duration::from_millis(100));
}
