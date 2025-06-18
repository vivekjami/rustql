pub mod handlers;

use crate::config::Settings;
use crate::utils::{generate_request_id, Result};
use std::sync::Arc;
use tracing::{info, instrument};
use warp::{Filter, Rejection, Reply};

pub struct Server {
    settings: Arc<Settings>,
}

impl Server {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: Arc::new(settings),
        }
    }

    #[instrument(skip(self))]
    pub async fn start(self) -> Result<()> {
        let settings = self.settings.clone();
        
        info!(
            "Starting RustQL server on {}:{}",
            settings.server.host, settings.server.port
        );

        // Build routes
        let routes = self.build_routes();

        // Start server
        let addr: std::net::SocketAddr = format!("{}:{}", settings.server.host, settings.server.port)
            .parse()
            .map_err(|e| crate::utils::RustQLError::Config(format!("Invalid server address: {}", e)))?;

        warp::serve(routes)
            .run(addr)
            .await;

        Ok(())
    }

    fn build_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization", "x-request-id"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"]);

        // Health check endpoint
        let health = warp::path("health")
            .and(warp::get())
            .and_then(handlers::handle_health);

        // GraphQL endpoint (placeholder for now)
        let graphql = warp::path("graphql")
            .and(warp::post())
            .and(self.with_request_id())
            .and(self.with_settings())
            .and(warp::body::json())
            .and_then(handlers::handle_graphql);

        // GraphQL playground
        let playground = warp::path("playground")
            .and(warp::get())
            .and_then(handlers::handle_playground);

        // Metrics endpoint
        let metrics = warp::path("metrics")
            .and(warp::get())
            .and_then(handlers::handle_metrics);

        health
            .or(graphql)
            .or(playground)
            .or(metrics)
            .with(self.with_logging())
            .with(cors)
            .recover(handlers::handle_rejection)
    }

    fn with_settings(&self) -> impl Filter<Extract = (Arc<Settings>,), Error = std::convert::Infallible> + Clone {
        let settings = self.settings.clone();
        warp::any().map(move || settings.clone())
    }

    fn with_request_id(&self) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
        warp::any().map(|| generate_request_id())
    }

    fn with_logging(&self) -> warp::log::Log<impl Fn(warp::log::Info) + Clone> {
        warp::log::custom(|info| {
            info!(
                method = %info.method(),
                path = %info.path(),
                status = %info.status(),
                elapsed = ?info.elapsed(),
                remote_addr = ?info.remote_addr(),
                "HTTP request processed"
            );
        })
    }
}