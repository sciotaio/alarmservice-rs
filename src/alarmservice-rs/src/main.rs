mod api;
mod config;
mod persistence;
mod tracing;

use std::net::SocketAddr;

use ::tracing::info;
use api::router::{router, AppState};
use clap::Parser;
use config::AppConfig;
use persistence::{database::init_connection, migration::Migrator};
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;
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
    Migrator::up(&db_conn, None)
        .await
        .expect("Migrations failed!");

    info!("Initialization done.");

    let state = AppState { conn: db_conn };

    let router = router(state);

    // start our server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port as u16));
    info!("Listening on port '{}'", config.server.port as u16);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to initialize tcp listener!");
    axum::serve(listener, router)
        .await
        .expect("Failed to start the server!");
}
