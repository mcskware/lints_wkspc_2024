//! Binary crate

use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::Layer as _;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

/// Initialize tracing with two layers:
/// 1. File layer: logs everything (TRACE and up) to a file
/// 2. Stdout layer: logs INFO and above to stdout
fn init_tracing() {
    // --- FILE LAYER: everything goes to file (TRACE and up) ---
    println!("Initializing tracing...");

    // changeme: update logs dir if needed
    let log_dir = "logs";

    // ensure the "logs" subdirectory exists
    std::fs::create_dir_all(log_dir).expect("Failed to create logs directory");

    // Logs will be in ./logs/app.log, rotated daily. Adjust as needed.
    let file_appender = rolling::daily(log_dir, "app.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    // Keep `_guard` somewhere (e.g. static or in AppState) if you want to ensure flush on shutdown.

    let file_layer = layer()
        .with_writer(file_writer)
        .with_ansi(false) // usually nicer for files
        .with_target(true)
        .with_level(true)
        .with_filter(EnvFilter::new("trace")); // everything

    // --- STDOUT LAYER: only INFO and above ---

    let stdout_layer = layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_filter(EnvFilter::new("debug,turn::this::span=off")); // INFO, WARN, ERROR

    // --- COMBINE LAYERS INTO A SINGLE SUBSCRIBER ---

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    info!("Tracing initialized");
}

/// Main function
fn main() {
    init_tracing();

    let greeting = changeme_lib::get_greeting_message();
    info!("{greeting}");
}
