
use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub rate_limiting: RateLimitConfig,
    pub apis: ApiConfig,
    pub security: SecurityConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub max_connections: usize,
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CacheConfig {
    pub redis_url: Option<String>,
    pub default_ttl: u64,
    pub max_size: String,
    pub compression: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub algorithm: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiConfig {
    pub rest: Vec<RestApiConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RestApiConfig {
    pub name: String,
    pub base_url: String,
    pub schema_url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    pub cors_origins: Vec<String>,
    pub jwt_secret: Option<String>,
    pub enable_introspection: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub prometheus_port: u16,
    pub tracing_enabled: bool,
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(config_path).required(false))
            .add_source(Environment::with_prefix("RUSTQL"))
            .build()?;
        
        s.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 5000,
                workers: 4,
                max_connections: 1000,
                timeout: 30,
            },
            cache: CacheConfig {
                redis_url: None,
                default_ttl: 300,
                max_size: "1GB".to_string(),
                compression: true,
            },
            rate_limiting: RateLimitConfig {
                requests_per_minute: 1000,
                burst_size: 50,
                algorithm: "sliding_window".to_string(),
            },
            apis: ApiConfig {
                rest: vec![],
            },
            security: SecurityConfig {
                cors_origins: vec!["*".to_string()],
                jwt_secret: None,
                enable_introspection: true,
            },
            metrics: MetricsConfig {
                enabled: true,
                prometheus_port: 9090,
                tracing_enabled: true,
            },
        }
    }
}
