use crate::core::config::Config;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init() {
    let config = Config::load().unwrap_or_default();
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.logging.level));

    let subscriber = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    if config.logging.json_output {
        let subscriber = subscriber.json().flatten_event(true);
        tracing_subscriber::registry()
            .with(filter)
            .with(subscriber)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(subscriber)
            .init();
    }
}
