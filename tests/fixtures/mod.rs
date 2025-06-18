use serde_json::Value;

pub fn sample_graphql_query() -> &'static str {
    r#"
    query {
        apiInfo {
            name
            version
            description
        }
    }
    "#
}

pub fn sample_rest_response() -> Value {
    serde_json::json!({
        "id": 1,
        "name": "Test User",
        "email": "test@example.com"
    })
}

pub fn complex_graphql_query() -> &'static str {
    r#"
    query GetUserWithPosts {
        user(id: 1) {
            id
            name
            email
            posts {
                id
                title
                body
            }
        }
    }
    "#
}
