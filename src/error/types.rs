
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RustQLError>;

#[derive(Error, Debug)]
pub enum RustQLError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    #[error("GraphQL error: {0}")]
    GraphQL(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Authentication failed: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Generic error: {0}")]
    Other(#[from] anyhow::Error),
}

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
            RustQLError::Timeout(_) => 504,
            RustQLError::Json(_) => 400,
            RustQLError::Io(_) => 500,
            RustQLError::Other(_) => 500,
        }
    }
}
