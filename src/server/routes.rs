
use axum::{
    routing::{get, post},
    Router,
    Extension,
};
use crate::server::handlers::{root, health_check, graphql_handler, metrics};
use crate::graphql::{playground, AppSchema};

pub fn create_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(playground::playground))
        .route("/metrics", get(metrics))
        .layer(Extension(schema))
}
