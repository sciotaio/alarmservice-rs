use tracing::subscriber::set_global_default;
use tracing_subscriber::layer::SubscriberExt;

use tracing_subscriber::{fmt, EnvFilter, Registry};

pub fn init_tracing(directives: &str) {
    println!("[Tracing]   Initializing tracing from config");

    // generate directives filter
    let directives_filter = EnvFilter::new(directives);

    // generate formatting layer
    let pretty_formatting_layer = fmt::Layer::new()
        .pretty()
        .with_target(true)
        .with_line_number(true)
        .with_thread_names(true);

    // generate subsciber and set it globally
    let subscriber = Registry::default()
        .with(directives_filter)
        .with(pretty_formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber!");
}
