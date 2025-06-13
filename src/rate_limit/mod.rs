
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    store: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    requests_per_minute: u32,
    window_size: Duration,
}

#[derive(Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            requests_per_minute,
            window_size: Duration::from_secs(60),
        }
    }
    
    pub async fn check_limit(&self, key: &str) -> bool {
        let mut store = self.store.write().await;
        let now = Instant::now();
        
        let entry = store.entry(key.to_string()).or_insert_with(|| RateLimitEntry {
            count: 0,
            window_start: now,
        });
        
        // Reset window if expired
        if now.duration_since(entry.window_start) >= self.window_size {
            entry.count = 0;
            entry.window_start = now;
        }
        
        if entry.count >= self.requests_per_minute {
            false
        } else {
            entry.count += 1;
            true
        }
    }
}
