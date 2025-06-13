
use axum::{
    http::StatusCode,
    response::{Json, Response},
};
use serde_json::json;
use crate::error::RustQLError;

pub async fn handle_error(err: RustQLError) -> Response {
    let status_code = StatusCode::from_u16(err.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    
    let body = Json(json!({
        "error": {
            "message": err.to_string(),
            "code": err.status_code()
        }
    }));
    
    (status_code, body).into_response()
}
