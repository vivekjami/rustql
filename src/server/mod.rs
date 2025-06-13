
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod websocket;

use crate::config::Settings;
use crate::error::Result;
use axum::{Router, Server as AxumServer};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct Server {
    app: Router,
    addr: SocketAddr,
}

impl Server {
    pub async fn new(settings: Settings) -> Result<Self> {
        let addr = format!("{}:{}", settings.server.host, settings.server.port)
            .parse::<SocketAddr>()?;
        
        let app = routes::create_routes(settings).await?
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive())
            );
        
        Ok(Self { app, addr })
    }
    
    pub async fn run(self) -> Result<()> {
        info!("Server starting on {}", self.addr);
        
        AxumServer::bind(&self.addr)
            .serve(self.app.into_make_service())
            .await?;
        
        Ok(())
    }
}
