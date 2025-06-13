
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CacheManager {
    store: Arc<RwLock<HashMap<String, CacheEntry>>>,
    default_ttl: Duration,
}

#[derive(Clone)]
struct CacheEntry {
    value: Value,
    expires_at: Instant,
}

impl CacheManager {
    pub fn new(default_ttl_secs: u64) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Duration::from_secs(default_ttl_secs),
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<Value> {
        let store = self.store.read().await;
        if let Some(entry) = store.get(key) {
            if entry.expires_at > Instant::now() {
                Some(entry.value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub async fn set(&self, key: String, value: Value, ttl: Option<Duration>) {
        let expires_at = Instant::now() + ttl.unwrap_or(self.default_ttl);
        let entry = CacheEntry { value, expires_at };
        
        let mut store = self.store.write().await;
        store.insert(key, entry);
    }
    
    pub async fn clear_expired(&self) {
        let mut store = self.store.write().await;
        let now = Instant::now();
        store.retain(|_, entry| entry.expires_at > now);
    }
}
