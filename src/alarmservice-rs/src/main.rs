mod config;
mod tracing;

use std::{thread, time::Duration};

use config::AppConfig;
use ::tracing::info;
use tracing::init_tracing;

fn main() {
    let config = AppConfig::parse().expect("Parsing config failed!");

    // init tracing
    let directives = &config.logging.levels.join(",");
    init_tracing(directives);

    info!("Initialization done.");
    info!("Starting server at port {} ...", config.server.port);

    thread::sleep(Duration::from_millis(100));
}
