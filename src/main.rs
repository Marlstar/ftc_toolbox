use tracing_subscriber::fmt::format::Format;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(match std::env::var("RUST_LOG").unwrap_or(String::from("INFO")).as_str() {
            "ERROR" => tracing::Level::ERROR,
            "WARN" => tracing::Level::WARN,
            "INFO" => tracing::Level::INFO,
            "DEBUG" => tracing::Level::DEBUG,
            "TRACE" => tracing::Level::TRACE,
            _ => tracing::Level::INFO,
        })
        .without_time()
        .event_format(Format::default()
            .compact()
            .without_time()
        ).init();

    if let Err(e) = ftc_toolbox::tools::adb::install::if_necessary() {
        tracing::error!("failed to install adb ({e:?})");
        return;
    }

    tracing::info!("shutting down");
}
