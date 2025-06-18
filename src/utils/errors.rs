use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustQLError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("GraphQL error: {0}")]
    GraphQL(String),

    #[error("REST API error: {message} (status: {status})")]
    RestApi { message: String, status: u16 },

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, RustQLError>;

impl RustQLError {
    pub fn status_code(&self) -> u16 {
        match self {
            RustQLError::Config(_) => 500,
            RustQLError::GraphQL(_) => 400,
            RustQLError::RestApi { status, .. } => *status,
            RustQLError::Cache(_) => 500,
            RustQLError::RateLimit(_) => 429,
            RustQLError::Auth(_) => 401,
            RustQLError::Validation(_) => 400,
            RustQLError::Internal(_) => 500,
            RustQLError::Network(_) => 502,
            RustQLError::Json(_) => 400,
            RustQLError::Redis(_) => 500,
            RustQLError::Io(_) => 500,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            RustQLError::Config(_) => "CONFIG_ERROR",
            RustQLError::GraphQL(_) => "GRAPHQL_ERROR",
            RustQLError::RestApi { .. } => "REST_API_ERROR",
            RustQLError::Cache(_) => "CACHE_ERROR",
            RustQLError::RateLimit(_) => "RATE_LIMIT_EXCEEDED",
            RustQLError::Auth(_) => "AUTHENTICATION_ERROR",
            RustQLError::Validation(_) => "VALIDATION_ERROR",
            RustQLError::Internal(_) => "INTERNAL_ERROR",
            RustQLError::Network(_) => "NETWORK_ERROR",
            RustQLError::Json(_) => "JSON_PARSE_ERROR",
            RustQLError::Redis(_) => "REDIS_ERROR",
            RustQLError::Io(_) => "IO_ERROR",
        }
    }
}
