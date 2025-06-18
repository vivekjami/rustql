use rustql::{Result, create_app};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Handle graceful shutdown
    let shutdown = tokio::signal::ctrl_c();

    // Create and start the application
    let app_result = tokio::select! {
        result = run_app() => result,
        _ = shutdown => {
            info!("Received shutdown signal, gracefully shutting down...");
            Ok(())
        }
    };

    if let Err(e) = app_result {
        error!("Application error: {}", e);
        std::process::exit(1);
    }

    info!("RustQL shut down successfully");
    Ok(())
}

async fn run_app() -> Result<()> {
    // Create application
    let server = create_app().await?;

    // Start server
    server.start().await?;

    Ok(())
}
