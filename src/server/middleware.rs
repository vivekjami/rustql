
use axum::{
    http::{Request, Response},
    middleware::Next,
    response::IntoResponse,
};
use tower::Service;
use std::time::Instant;
use tracing::info;

pub async fn request_logging<B>(
    request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    info!(
        method = %method,
        uri = %uri,
        duration_ms = duration.as_millis(),
        "Request processed"
    );
    
    response
}
