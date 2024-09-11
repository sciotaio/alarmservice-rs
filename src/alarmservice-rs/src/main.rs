mod config;
mod tracing;
mod persistence;

use std::{thread, time::Duration};

use clap::Parser;
use config::AppConfig;
use persistence::{database::init_connection, migration::Migrator};
use sea_orm_migration::MigratorTrait;
use ::tracing::info;
use tracing::init_tracing;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("config/default"))]
    config: String,
}

#[tokio::main]
async fn main() {
    // parse our args and init 'config_file_name' (from default or provided arg)
    let args = Args::parse();

    // parse our app config
    let config = AppConfig::parse(args.config).expect("Parsing config failed!");

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
