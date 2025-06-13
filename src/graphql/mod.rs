
pub mod schema;
pub mod playground;

pub use schema::{AppSchema, create_schema};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;

pub async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
