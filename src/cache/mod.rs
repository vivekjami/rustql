
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, Duration};
use serde_json::Value;

#[derive(Clone)]
pub struct CacheItem {
    value: Value,
    expires_at: SystemTime,
}

#[derive(Clone)]
pub struct MemoryCache {
    store: Arc<RwLock<HashMap<String, CacheItem>>>,
    default_ttl: Duration,
}

impl MemoryCache {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<Value> {
        let store = self.store.read().await;
        if let Some(item) = store.get(key) {
            if SystemTime::now() < item.expires_at {
                Some(item.value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub async fn set(&self, key: String, value: Value, ttl: Option<Duration>) {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let expires_at = SystemTime::now() + ttl;
        
        let item = CacheItem { value, expires_at };
        let mut store = self.store.write().await;
        store.insert(key, item);
    }
    
    pub async fn delete(&self, key: &str) {
        let mut store = self.store.write().await;
        store.remove(key);
    }
    
    pub async fn cleanup_expired(&self) {
        let mut store = self.store.write().await;
        let now = SystemTime::now();
        store.retain(|_, item| now < item.expires_at);
    }
}
