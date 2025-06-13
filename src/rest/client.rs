
use std::collections::HashMap;
use std::time::Duration;
use reqwest::{Client, Method, Response};
use serde_json::Value;
use tokio::time::timeout;
use crate::error::Result;

#[derive(Clone)]
pub struct RestClient {
    client: Client,
    apis: HashMap<String, String>,
    timeout: Duration,
}

impl RestClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            apis: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }
    
    pub fn add_api(&mut self, name: String, base_url: String) {
        self.apis.insert(name, base_url);
    }
    
    pub async fn execute_request(
        &self,
        endpoint: &str,
        method: &str,
        headers: Option<Value>,
    ) -> Result<Value> {
        let url = self.build_url(endpoint)?;
        let method = Method::from_bytes(method.as_bytes())
            .map_err(|_| crate::error::RustQLError::Validation("Invalid HTTP method".to_string()))?;
            
        let mut request = self.client.request(method, &url);
        
        if let Some(headers_obj) = headers {
            if let Some(headers_map) = headers_obj.as_object() {
                for (key, value) in headers_map {
                    if let Some(value_str) = value.as_str() {
                        request = request.header(key, value_str);
                    }
                }
            }
        }
        
        let response = timeout(self.timeout, request.send()).await
            .map_err(|_| crate::error::RustQLError::Timeout("Request timeout".to_string()))?
            .map_err(crate::error::RustQLError::HttpClient)?;
            
        self.handle_response(response).await
    }
    
    pub async fn execute_mutation(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<Value>,
        headers: Option<Value>,
    ) -> Result<Value> {
        let url = self.build_url(endpoint)?;
        let method = Method::from_bytes(method.as_bytes())
            .map_err(|_| crate::error::RustQLError::Validation("Invalid HTTP method".to_string()))?;
            
        let mut request = self.client.request(method, &url);
        
        if let Some(body) = body {
            request = request.json(&body);
        }
        
        if let Some(headers_obj) = headers {
            if let Some(headers_map) = headers_obj.as_object() {
                for (key, value) in headers_map {
                    if let Some(value_str) = value.as_str() {
                        request = request.header(key, value_str);
                    }
                }
            }
        }
        
        let response = timeout(self.timeout, request.send()).await
            .map_err(|_| crate::error::RustQLError::Timeout("Request timeout".to_string()))?
            .map_err(crate::error::RustQLError::HttpClient)?;
            
        self.handle_response(response).await
    }
    
    fn build_url(&self, endpoint: &str) -> Result<String> {
        // Simple URL building - in production you'd want more sophisticated routing
        if endpoint.starts_with("http") {
            Ok(endpoint.to_string())
        } else {
            // Use first available API as default
            let base_url = self.apis.values().next()
                .ok_or_else(|| crate::error::RustQLError::Validation("No APIs configured".to_string()))?;
            Ok(format!("{}{}", base_url, endpoint))
        }
    }
    
    async fn handle_response(&self, response: Response) -> Result<Value> {
        if response.status().is_success() {
            let json: Value = response.json().await
                .map_err(crate::error::RustQLError::HttpClient)?;
            Ok(json)
        } else {
            Err(crate::error::RustQLError::HttpClient(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ))
        }
    }
}
