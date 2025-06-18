use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub rate_limiting: RateLimitConfig,
    pub apis: ApisConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_connections: Option<usize>,
    pub request_timeout: Option<u64>,
    pub enable_playground: bool,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis_url: Option<String>,
    pub default_ttl: u64,
    pub max_size: String,
    pub enable_compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enable_per_ip: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApisConfig {
    pub rest: Vec<RestApiConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestApiConfig {
    pub name: String,
    pub base_url: String,
    pub schema_url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
    pub retry_attempts: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub metrics_port: u16,
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: None,
                max_connections: Some(1000),
                request_timeout: Some(30),
                enable_playground: true,
                cors_origins: vec!["*".to_string()],
            },
            cache: CacheConfig {
                redis_url: None,
                default_ttl: 300,
                max_size: "100MB".to_string(),
                enable_compression: true,
            },
            rate_limiting: RateLimitConfig {
                requests_per_minute: 1000,
                burst_size: 50,
                enable_per_ip: true,
            },
            apis: ApisConfig {
                rest: vec![],
            },
            monitoring: MonitoringConfig {
                enable_metrics: true,
                enable_tracing: true,
                metrics_port: 9090,
                log_level: "info".to_string(),
            },
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("RUSTQL"));

        // Override with environment variables
        if let Ok(port) = std::env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        let settings = builder.build()?;
        settings.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.server.port == 0 {
            return Err("Server port cannot be 0".to_string());
        }

        if self.rate_limiting.requests_per_minute == 0 {
            return Err("Rate limit requests per minute cannot be 0".to_string());
        }

        for api in &self.apis.rest {
            if api.base_url.is_empty() {
                return Err(format!("Base URL for API '{}' cannot be empty", api.name));
            }
        }

        Ok(())
    }
}