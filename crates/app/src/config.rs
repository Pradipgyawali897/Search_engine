use db::PostgresConfig;
use indexer::config::{
    DEFAULT_INDEX_PATH, DEFAULT_JUNK_URLS_PATH, DEFAULT_SEED_PATH, DEFAULT_VISITABLE_URLS_PATH,
    RuntimePaths,
};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub paths: RuntimePaths,
    pub concurrency: usize,
    pub database: Option<PostgresConfig>,
}

pub fn load_app_config() -> AppConfig {
    AppConfig {
        paths: RuntimePaths::new(
            path_from_env("PERNOX_INDEX_PATH", DEFAULT_INDEX_PATH),
            path_from_env("PERNOX_SEED_FILE", DEFAULT_SEED_PATH),
            path_from_env("PERNOX_VISITABLE_URLS_PATH", DEFAULT_VISITABLE_URLS_PATH),
            path_from_env("PERNOX_JUNK_URLS_PATH", DEFAULT_JUNK_URLS_PATH),
        ),
        concurrency: env::var("PERNOX_CONCURRENCY")
            .ok()
            .and_then(|v| v.parse().ok())
            .filter(|&n| n > 0)
            .unwrap_or(8),
        database: load_database_config(),
    }
}

pub fn load_database_config() -> Option<PostgresConfig> {
    let database_url = env::var("PERNOX_DATABASE_URL").ok()?;
    if database_url.trim().is_empty() {
        return None;
    }

    let mut config = PostgresConfig::new(database_url);

    if let Some(schema) = non_empty_env("PERNOX_DATABASE_SCHEMA") {
        config.schema = schema;
    }

    if let Some(max_connections) = positive_env_u32("PERNOX_DATABASE_MAX_CONNECTIONS") {
        config.max_connections = max_connections;
    }

    if let Some(min_connections) = positive_env_u32("PERNOX_DATABASE_MIN_CONNECTIONS") {
        config.min_connections = min_connections;
    }

    if let Some(timeout_secs) = positive_env_u64("PERNOX_DATABASE_ACQUIRE_TIMEOUT_SECS") {
        config.acquire_timeout = Duration::from_secs(timeout_secs);
    }

    Some(config)
}

fn path_from_env(key: &str, default: &str) -> PathBuf {
    env::var_os(key)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default))
}

fn non_empty_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn positive_env_u32(key: &str) -> Option<u32> {
    env::var(key).ok()?.parse().ok().filter(|value| *value > 0)
}

fn positive_env_u64(key: &str) -> Option<u64> {
    env::var(key).ok()?.parse().ok().filter(|value| *value > 0)
}
