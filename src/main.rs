
use anyhow::Result;
use clap::Parser;
use rustql::config::Settings;
use rustql::server::Server;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let log_level = match cli.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();
    
    info!("Starting RustQL GraphQL-to-REST Gateway");
    
    // Load configuration
    let settings = Settings::new(&cli.config)?;
    info!("Configuration loaded from: {}", cli.config);
    
    // Start server
    let server = Server::new(settings).await?;
    server.run().await?;
    
    Ok(())
}
