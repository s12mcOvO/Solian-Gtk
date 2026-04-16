mod core;
mod ui;

use adw::prelude::*;
use adw::Application;
use std::process;
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

fn main() {
    setup_logging();

    info!("Starting Solian GTK4 Application");

    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| {
                panic_info
                    .payload()
                    .downcast_ref::<String>()
                    .cloned()
            })
            .unwrap_or_else(|| "Unknown panic".to_string());

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        error!(
            "Application panic: {} at {}",
            payload, location
        );

        process::exit(1);
    }));

    let app = Application::builder()
        .application_id("app.solian.Solian")
        .build();

    app.connect_activate(|app| {
        info!("Application activated");
        if let Err(e) = ui::application::setup_and_run(app) {
            error!("Failed to setup application: {}", e);
            std::process::exit(1);
        }
    });

    info!("Running application");
    app.run();
}