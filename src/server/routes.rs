
use crate::config::Settings;
use crate::error::Result;
use crate::server::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub async fn create_routes(settings: Settings) -> Result<Router> {
    let router = Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health_check))
        .route("/graphql", post(handlers::graphql_handler))
        .route("/graphql", get(handlers::graphql_playground))
        .route("/metrics", get(handlers::metrics));
    
    Ok(router)
}
