use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub port: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Logging {
    pub levels: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct Postgres {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub server: Server,
    pub logging: Logging,
    pub postgres: Postgres,
}

impl AppConfig {
    pub fn parse() -> Result<Self, ConfigError> {
        let current_dir = std::env::current_dir().expect("Could not determine CWD");
        let config_file_path = "config/default";
        
        println!("[AppConfig] CWD: '{current_dir:?}'");
        println!("[AppConfig] Loading config relative to CWD from: '{config_file_path}'");

        let app_config = Config::builder()
            .add_source(File::with_name(config_file_path).required(true))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        app_config.try_deserialize()
    }
}
