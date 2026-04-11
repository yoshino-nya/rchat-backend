use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,chat=debug"));

    tracing_subscriber::registry()
        .with(fmt::layer().pretty().with_target(false))
        .with(env_filter)
        .init();
}
