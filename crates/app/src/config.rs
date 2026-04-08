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
    let database_url = first_non_empty_env(&["PERNOX_DATABASE_URL", "DATABASE_URL"])?;
    if database_url.trim().is_empty() {
        return None;
    }

    let mut config = PostgresConfig::new(database_url);

    if let Some(schema) = first_non_empty_env(&["PERNOX_DATABASE_SCHEMA", "DATABASE_SCHEMA"]) {
        config.schema = schema;
    }

    if let Some(max_connections) = first_positive_env_u32(&[
        "PERNOX_DATABASE_MAX_CONNECTIONS",
        "DATABASE_MAX_CONNECTIONS",
    ]) {
        config.max_connections = max_connections;
    }

    if let Some(min_connections) = first_positive_env_u32(&[
        "PERNOX_DATABASE_MIN_CONNECTIONS",
        "DATABASE_MIN_CONNECTIONS",
    ]) {
        config.min_connections = min_connections;
    }

    if let Some(timeout_secs) = first_positive_env_u64(&[
        "PERNOX_DATABASE_ACQUIRE_TIMEOUT_SECS",
        "DATABASE_ACQUIRE_TIMEOUT_SECS",
    ]) {
        config.acquire_timeout = Duration::from_secs(timeout_secs);
    }

    Some(config)
}

fn path_from_env(key: &str, default: &str) -> PathBuf {
    let path = env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default));

    resolve_runtime_path(path)
}

fn non_empty_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn first_non_empty_env(keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| non_empty_env(key))
}

fn positive_env_u32(key: &str) -> Option<u32> {
    env::var(key).ok()?.parse().ok().filter(|value| *value > 0)
}

fn first_positive_env_u32(keys: &[&str]) -> Option<u32> {
    keys.iter().find_map(|key| positive_env_u32(key))
}

fn positive_env_u64(key: &str) -> Option<u64> {
    env::var(key).ok()?.parse().ok().filter(|value| *value > 0)
}

fn first_positive_env_u64(keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| positive_env_u64(key))
}

fn resolve_runtime_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        return path;
    }

    match env::var("PERNOX_APP_BASE_DIR") {
        Ok(base_dir) if !base_dir.trim().is_empty() => PathBuf::from(base_dir.trim()).join(path),
        _ => path,
    }
}
