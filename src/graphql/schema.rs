
use async_graphql::{Schema, EmptySubscription, Object, Result, Context};
use serde_json::Value;
use crate::rest::RestClient;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn health(&self) -> &str {
        "OK"
    }
    
    async fn rest_query(
        &self,
        ctx: &Context<'_>,
        endpoint: String,
        method: Option<String>,
        headers: Option<Value>,
    ) -> Result<Value> {
        let client = ctx.data::<RestClient>()?;
        let method = method.unwrap_or_else(|| "GET".to_string());
        
        client.execute_request(&endpoint, &method, headers).await
            .map_err(|e| async_graphql::Error::new(format!("REST request failed: {}", e)))
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn rest_mutation(
        &self,
        ctx: &Context<'_>,
        endpoint: String,
        method: String,
        body: Option<Value>,
        headers: Option<Value>,
    ) -> Result<Value> {
        let client = ctx.data::<RestClient>()?;
        
        client.execute_mutation(&endpoint, &method, body, headers).await
            .map_err(|e| async_graphql::Error::new(format!("REST mutation failed: {}", e)))
    }
}

pub fn create_schema(rest_client: RestClient) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(rest_client)
        .finish()
}
