
use std::future::Future;
use crate::error::{Result, RustQLError};

pub async fn with_retry<F, Fut, T>(
    operation: F,
    max_retries: u32,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut last_error = None;
    
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                last_error = Some(err);
                if attempt < max_retries {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * (attempt + 1) as u64)).await;
                }
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| RustQLError::Internal("Unknown error during retry".to_string())))
}
