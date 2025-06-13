use axum::{
    http::StatusCode,
    response::Json,
    Extension,
};
use serde_json::{json, Value};
use crate::graphql::AppSchema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

pub async fn root() -> Json<Value> {
    Json(json!({
        "name": "RustQL",
        "version": "0.1.0",
        "description": "High-performance GraphQL-to-REST API Gateway",
        "endpoints": {
            "graphql": "/graphql",
            "playground": "/playground",
            "health": "/health",
            "metrics": "/metrics"
        }
    }))
}

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn metrics() -> Json<Value> {
    // TODO: Implement actual metrics collection
    Json(json!({
        "metrics": {
            "requests_total": 0,
            "requests_per_second": 0,
            "average_latency_ms": 0,
            "cache_hit_ratio": 0.0,
            "active_connections": 0
        }
    }))
}

use axum::response::Html;

pub async fn graphql_playground() -> Html<String> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>RustQL GraphQL Playground</title>
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
        </head>
        <body>
            <div id="root">
                <style>
                    body { margin: 0; font-family: Open Sans,sans-serif; overflow: hidden; }
                    #root { height: 100vh; }
                </style>
                <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
                <script>
                    window.addEventListener('load', function (event) {
                        GraphQLPlayground.init(document.getElementById('root'), {
                            endpoint: '/graphql'
                        })
                    })
                </script>
            </div>
        </body>
        </html>
        "#.to_string()
    )
}