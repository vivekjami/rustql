use async_graphql::{Schema, Object, Context, Result, SimpleObject};
use serde_json::Value;
use crate::rest::RestClient;

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get user by ID from REST API
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Value> {
        let client = ctx.data::<RestClient>().map_err(|_| "RestClient not found")?;
        let endpoint = format!("/users/{}", id);

        match client.execute_request(&endpoint, "GET", None).await {
            Ok(data) => Ok(data),
            Err(_) => Ok(serde_json::json!({"error": "User not found"}))
        }
    }

    /// Get posts from REST API
    async fn posts(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Value> {
        let client = ctx.data::<RestClient>().map_err(|_| "RestClient not found")?;
        let mut endpoint = "/posts".to_string();

        if let Some(limit) = limit {
            endpoint.push_str(&format!("?_limit={}", limit));
        }

        match client.execute_request(&endpoint, "GET", None).await {
            Ok(data) => Ok(data),
            Err(_) => Ok(serde_json::json!({"error": "Posts not found"}))
        }
    }

    /// Health check endpoint
    async fn health(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new post
    async fn create_post(&self, ctx: &Context<'_>, title: String, body: String, user_id: i32) -> Result<Value> {
        let client = ctx.data::<RestClient>().map_err(|_| "RestClient not found")?;

        let post_data = serde_json::json!({
            "title": title,
            "body": body,
            "userId": user_id
        });

        match client.execute_mutation("/posts", "POST", Some(post_data), None).await {
            Ok(data) => Ok(data),
            Err(_) => Ok(serde_json::json!({"error": "Failed to create post"}))
        }
    }

    /// Update a post
    async fn update_post(&self, ctx: &Context<'_>, id: i32, title: Option<String>, body: Option<String>) -> Result<Value> {
        let client = ctx.data::<RestClient>().map_err(|_| "RestClient not found")?;

        let mut update_data = serde_json::Map::new();
        if let Some(title) = title {
            update_data.insert("title".to_string(), serde_json::Value::String(title));
        }
        if let Some(body) = body {
            update_data.insert("body".to_string(), serde_json::Value::String(body));
        }

        let endpoint = format!("/posts/{}", id);
        match client.execute_mutation(&endpoint, "PATCH", Some(serde_json::Value::Object(update_data)), None).await {
            Ok(data) => Ok(data),
            Err(_) => Ok(serde_json::json!({"error": "Failed to update post"}))
        }
    }
}