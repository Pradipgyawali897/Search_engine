use crate::error::AppError;
use db::schema::DEFAULT_POSTGRES_SCHEMA;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AbWebConfig {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub database_schema: String,
    pub cache_ttl: Duration,
    pub refresh_interval: Duration,
    pub default_limit: u32,
    pub max_limit: u32,
}

impl AbWebConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            bind_addr: parse_socket_addr(env_or("AB_BIND_ADDR", "127.0.0.1:3001"))?,
            database_url: required_env_any(&["DATABASE_URL", "PERNOX_DATABASE_URL"])?,
            database_schema: env_first(
                &["DATABASE_SCHEMA", "PERNOX_DATABASE_SCHEMA"],
                DEFAULT_POSTGRES_SCHEMA,
            ),
            cache_ttl: Duration::from_secs(env_u64("AB_CACHE_TTL_SECS", 90)),
            refresh_interval: Duration::from_secs(env_u64("AB_REFRESH_INTERVAL_SECS", 180)),
            default_limit: env_u32("AB_DEFAULT_LIMIT", 18),
            max_limit: env_u32("AB_MAX_LIMIT", 60),
        })
    }
}

fn required_env_any(keys: &[&str]) -> Result<String, AppError> {
    keys.iter().find_map(|key| optional_env(key)).ok_or_else(|| {
        AppError::config(format!(
            "missing required environment variable; set one of {}",
            keys.join(", ")
        ))
    })
}

fn optional_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(normalize_env_value)
        .filter(|value| !value.is_empty())
}

fn env_or(key: &str, default: &str) -> String {
    optional_env(key).unwrap_or_else(|| default.to_string())
}

fn env_first(keys: &[&str], default: &str) -> String {
    keys.iter()
        .find_map(|key| optional_env(key))
        .unwrap_or_else(|| default.to_string())
}

fn env_u64(key: &str, default: u64) -> u64 {
    optional_env(key)
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn env_u32(key: &str, default: u32) -> u32 {
    optional_env(key)
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn parse_socket_addr(raw: String) -> Result<SocketAddr, AppError> {
    raw.parse()
        .map_err(|error| AppError::config(format!("invalid AB_BIND_ADDR '{raw}': {error}")))
}

fn normalize_env_value(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let quoted =
            (trimmed.starts_with('"') && trimmed.ends_with('"'))
                || (trimmed.starts_with('\'') && trimmed.ends_with('\''));
        if quoted {
            return trimmed[1..trimmed.len() - 1].trim().to_string();
        }
    }

    trimmed.to_string()
}
