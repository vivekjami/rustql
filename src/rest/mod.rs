pub mod adapter;
pub mod client;

// Placeholder for Day 2 implementation
pub struct RestClient {
    base_url: String,
}

impl RestClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get(&self, path: &str) -> crate::utils::Result<serde_json::Value> {
        // Placeholder implementation
        Ok(serde_json::json!({
            "message": "REST client placeholder",
            "url": format!("{}/{}", self.base_url, path),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}
