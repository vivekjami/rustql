
use reqwest::{Client, Method, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use crate::error::{Result, RustQLError};

#[derive(Clone)]
pub struct RestClient {
    client: Client,
    base_urls: HashMap<String, String>,
}

impl RestClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_urls: HashMap::new(),
        }
    }
    
    pub fn add_api(&mut self, name: String, base_url: String) {
        self.base_urls.insert(name, base_url);
    }
    
    pub async fn execute_request(
        &self,
        endpoint: &str,
        method: &str,
        headers: Option<Value>,
    ) -> Result<Value> {
        let method = Method::from_str(method)
            .map_err(|_| RustQLError::Validation(format!("Invalid HTTP method: {}", method)))?;
        
        let mut request = self.client.request(method, endpoint);
        
        if let Some(headers_obj) = headers {
            if let Value::Object(map) = headers_obj {
                for (key, value) in map {
                    if let Value::String(val) = value {
                        request = request.header(&key, val);
                    }
                }
            }
        }
        
        let response = request.send().await?;
        let json: Value = response.json().await?;
        
        Ok(json)
    }
    
    pub async fn execute_mutation(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<Value>,
        headers: Option<Value>,
    ) -> Result<Value> {
        let method = Method::from_str(method)
            .map_err(|_| RustQLError::Validation(format!("Invalid HTTP method: {}", method)))?;
        
        let mut request = self.client.request(method, endpoint);
        
        if let Some(body_data) = body {
            request = request.json(&body_data);
        }
        
        if let Some(headers_obj) = headers {
            if let Value::Object(map) = headers_obj {
                for (key, value) in map {
                    if let Value::String(val) = value {
                        request = request.header(&key, val);
                    }
                }
            }
        }
        
        let response = request.send().await?;
        let json: Value = response.json().await?;
        
        Ok(json)
    }
}
