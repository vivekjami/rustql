
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use crate::config::Settings;
use crate::error::Result;
use crate::graphql::create_schema;
use crate::rest::RestClient;

pub mod handlers;
pub mod middleware;
pub mod routes;

pub struct Server {
    app: Router,
    addr: SocketAddr,
}

impl Server {
    pub async fn new(settings: Settings) -> Result<Self> {
        // Create REST client
        let mut rest_client = RestClient::new();
        
        // Add configured APIs
        for api in &settings.apis {
            rest_client.add_api(api.name.clone(), api.base_url.clone());
        }
        
        // Create GraphQL schema
        let schema = create_schema(rest_client);
        
        // Create router
        let app = routes::create_router(schema);
        
        let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
        
        Ok(Self { app, addr })
    }
    
    pub async fn run(self) -> Result<()> {
        info!("🚀 RustQL server starting on {}", self.addr);
        
        let listener = TcpListener::bind(self.addr).await?;
        
        info!("✅ Server running at http://{}", self.addr);
        info!("📊 GraphQL Playground available at http://{}/playground", self.addr);
        
        axum::serve(listener, self.app).await?;
        
        Ok(())
    }
}
