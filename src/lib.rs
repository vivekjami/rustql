pub mod cache;
pub mod config;
pub mod graphql;
pub mod metrics;
pub mod rate_limit;
pub mod rest;
pub mod server;
pub mod utils;

pub use config::Settings;
pub use server::Server;
pub use utils::{Result, RustQLError};

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_tracing(log_level: &str) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .map_err(|e| RustQLError::Config(format!("Invalid log level: {}", e)))?;

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).compact())
        .with(filter)
        .init();

    Ok(())
}

pub async fn create_app() -> Result<Server> {
    // Load configuration
    let settings = Settings::load()
        .map_err(|e| RustQLError::Config(format!("Failed to load configuration: {}", e)))?;

    // Validate configuration
    settings.validate().map_err(|e| RustQLError::Config(e))?;

    // Initialize tracing
    init_tracing(&settings.monitoring.log_level)?;

    tracing::info!("RustQL starting up...");
    tracing::info!("Configuration loaded successfully");

    // Create server
    let server = Server::new(settings);

    Ok(server)
}
