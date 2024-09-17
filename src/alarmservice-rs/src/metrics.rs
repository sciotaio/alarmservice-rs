use axum_prometheus::metrics::{counter, Counter};

#[derive(Clone)]
pub struct AppMetrics {
    pub events_counter: Counter,
}

impl AppMetrics {
    pub fn init() -> Self {
        // initialize metrics
        let events_counter = counter!("events_counter");

        // initialize AppMetrics
        Self { events_counter }
    }
}
