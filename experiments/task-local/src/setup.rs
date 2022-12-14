use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup_tracing() {
    let filter_layer: EnvFilter = format!("warn,{}=trace", env!("CARGO_CRATE_NAME"))
        .parse()
        .unwrap();
    let fmt_layer = tracing_subscriber::fmt::layer();
    // let fmt_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
