use crate::config::Settings;
use async_graphql::{Context, EmptySubscription, Object, Result, Schema, SimpleObject};
use std::sync::Arc;

pub type RustQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub uptime: String,
}

#[derive(SimpleObject)]
pub struct SystemStatus {
    pub status: String,
    pub timestamp: String,
    pub request_count: i32,
    pub active_connections: i32,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get API information
    async fn api_info(&self) -> ApiInfo {
        ApiInfo {
            name: "RustQL".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "High-Performance GraphQL-to-REST API Gateway".to_string(),
            uptime: format!(
                "{:?}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
            ),
        }
    }

    /// Get system status
    async fn system_status(&self) -> SystemStatus {
        SystemStatus {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            request_count: 0,      // Will be implemented with metrics
            active_connections: 0, // Will be implemented with metrics
        }
    }

    /// Health check endpoint
    async fn health(&self) -> String {
        "OK".to_string()
    }

    /// Echo query for testing
    async fn echo(&self, message: String) -> String {
        format!("Echo: {}", message)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Test mutation
    async fn test_mutation(&self, input: String) -> String {
        format!("Processed: {}", input)
    }
}

pub fn create_schema(settings: Arc<Settings>) -> RustQLSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(settings)
        .finish()
}
