use std::time::Duration;

use sea_orm::{ConnectOptions, DatabaseConnection};
use tracing::{debug, info};

use crate::config::Postgres;

pub async fn init_connection(postgres: Postgres) -> DatabaseConnection {
    let db_user = postgres.user;
    let db_password = postgres.password;
    let db_hostname = postgres.host;
    let db_port = postgres.port;
    let db_name = postgres.database;
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_hostname, db_port, db_name
    );

    debug!(
        "Connecting to db postgres://{}:{}/{}...",
        &db_hostname, &db_port, &db_name
    );

    // setup connection
    let mut conn_options = ConnectOptions::new(db_url);
    conn_options.connect_timeout(Duration::from_secs(3))
        .sqlx_logging(true);

    // try to connect to db
    let connection = sea_orm::Database::connect(conn_options)
        .await
        .expect("Database connection failed!");
    info!("Connection with db established.");

    connection
}
