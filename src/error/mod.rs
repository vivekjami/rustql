
pub mod types;
pub mod handlers;
pub mod recovery;

pub use types::{RustQLError, Result};

impl RustQLError {
    pub fn status_code(&self) -> u16 {
        match self {
            RustQLError::Config(_) => 500,
            RustQLError::HttpClient(_) => 502,
            RustQLError::GraphQL(_) => 400,
            RustQLError::Cache(_) => 500,
            RustQLError::RateLimit => 429,
            RustQLError::Auth(_) => 401,
            RustQLError::Validation(_) => 400,
            RustQLError::Internal(_) => 500,
            RustQLError::ServiceUnavailable(_) => 503,
            RustQLError::Timeout(_) => 408,
            RustQLError::Json(_) => 400,
            RustQLError::Io(_) => 500,
            RustQLError::Other(_) => 500,
        }
    }
}
