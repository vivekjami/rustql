
use std::env;

pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_env_var_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_env_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .map(|v| v.parse().unwrap_or(default))
        .unwrap_or(default)
}

pub fn get_env_u16(key: &str, default: u16) -> u16 {
    env::var(key)
        .map(|v| v.parse().unwrap_or(default))
        .unwrap_or(default)
}
