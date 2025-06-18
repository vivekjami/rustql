pub mod errors;

pub use errors::{Result, RustQLError};

use uuid::Uuid;

pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn format_duration(duration: std::time::Duration) -> String {
    if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}μs", duration.as_micros())
    }
}

pub fn sanitize_query(query: &str) -> String {
    query
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ")
}
