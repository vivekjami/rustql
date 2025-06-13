
use axum::{
    routing::{get, post},
    Router,
    Extension,
};
use tower_http::cors::CorsLayer;
use crate::graphql::AppSchema;
use crate::server::handlers;

pub fn create_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health_check))
        .route("/graphql", post(handlers::graphql_handler))
        .route("/playground", get(handlers::graphql_playground))
        .route("/metrics", get(handlers::metrics))
        .layer(Extension(schema))
        .layer(CorsLayer::permissive())
}
