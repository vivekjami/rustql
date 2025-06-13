
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, Duration};

#[derive(Clone)]
pub struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, Vec<SystemTime>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window,
        }
    }
    
    pub async fn check_rate_limit(&self, key: &str) -> bool {
        let mut limits = self.limits.write().await;
        let now = SystemTime::now();
        let cutoff = now - self.window;
        
        let requests = limits.entry(key.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests
        requests.retain(|&time| time > cutoff);
        
        if requests.len() >= self.max_requests {
            false
        } else {
            requests.push(now);
            true
        }
    }
    
    pub async fn cleanup_old_entries(&self) {
        let mut limits = self.limits.write().await;
        let cutoff = SystemTime::now() - self.window;
        
        limits.retain(|_, requests| {
            requests.retain(|&time| time > cutoff);
            !requests.is_empty()
        });
    }
}
