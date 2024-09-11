mod config;
mod tracing;
mod persistence;

use std::{thread, time::Duration};

use config::AppConfig;
use persistence::{database::init_connection, migration::Migrator};
use sea_orm_migration::MigratorTrait;
use ::tracing::info;
use tracing::init_tracing;

#[tokio::main]
async fn main() {
    let config = AppConfig::parse().expect("Parsing config failed!");

    // init tracing
    let directives = &config.logging.levels.join(",");
    init_tracing(directives);

    // init DB
    let db_conn = init_connection(config.postgres).await;
    
    // run migrations
    Migrator::up(&db_conn, None).await.expect("Migrations failed!");

    info!("Initialization done.");
    info!("Starting server at port {} ...", config.server.port);

    thread::sleep(Duration::from_secs(100));
}
