mod config;
use config::AppConfig;
fn main() {
    let config = AppConfig::parse().expect("Parsing config failed!");

}
