use crate::config::Settings;
use serde_json::{Value, json};
use std::convert::Infallible;
use std::sync::Arc;
use tracing::{error, info, instrument};
use warp::{Rejection, Reply, http::StatusCode};

#[instrument]
pub async fn handle_health() -> Result<impl Reply, Rejection> {
    let response = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "service": "rustql"
    });

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

#[instrument(skip(_settings, _body))]
pub async fn handle_graphql(
    request_id: String,
    _settings: Arc<Settings>,
    _body: Value,
) -> Result<impl Reply, Rejection> {
    info!(request_id = %request_id, "Processing GraphQL request");

    // For now, return a placeholder response
    // This will be replaced with actual GraphQL execution in the next step
    let response = json!({
        "data": {
            "message": "GraphQL endpoint is ready",
            "request_id": request_id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

#[instrument]
pub async fn handle_playground() -> Result<impl Reply, Rejection> {
    let playground_html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>RustQL GraphQL Playground</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
</head>
<body>
    <div id="root">
        <style>
            body { margin: 0; font-family: "Open Sans", sans-serif; overflow: hidden; }
            #root { height: 100vh; }
        </style>
    </div>
    <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
    <script>
        window.addEventListener('load', function (event) {
            GraphQLPlayground.init(document.getElementById('root'), {
                endpoint: '/graphql',
                settings: {
                    'general.betaUpdates': false,
                    'editor.theme': 'dark',
                    'editor.reuseHeaders': true,
                    'tracing.hideTracingResponse': true,
                    'editor.fontSize': 14,
                    'editor.fontFamily': '"Source Code Pro", "Consolas", "Inconsolata", "Droid Sans Mono", "Monaco", monospace',
                    'request.credentials': 'omit',
                }
            })
        })
    </script>
</body>
</html>
    "#;

    Ok(warp::reply::with_header(
        playground_html,
        "content-type",
        "text/html",
    ))
}

#[instrument]
pub async fn handle_metrics() -> Result<impl Reply, Rejection> {
    // Placeholder for Prometheus metrics
    let metrics = "# RustQL Metrics\n# Metrics will be implemented in Day 4\nrustql_info{version=\"0.1.0\"} 1\n";

    Ok(warp::reply::with_header(
        metrics,
        "content-type",
        "text/plain; version=0.0.4; charset=utf-8",
    ))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message, error_code) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found", "NOT_FOUND")
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, "Invalid JSON body", "INVALID_JSON")
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed",
            "METHOD_NOT_ALLOWED",
        )
    } else {
        error!("Unhandled rejection: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "INTERNAL_ERROR",
        )
    };

    let json = json!({
        "error": {
            "code": error_code,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    Ok(warp::reply::with_status(warp::reply::json(&json), code))
}
