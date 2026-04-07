use indexer::config::{
    DEFAULT_INDEX_PATH, DEFAULT_JUNK_URLS_PATH, DEFAULT_SEED_PATH, DEFAULT_VISITABLE_URLS_PATH,
    RuntimePaths,
};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub paths: RuntimePaths,
    pub concurrency: usize,
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
    }
}

fn path_from_env(key: &str, default: &str) -> PathBuf {
    env::var_os(key)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default))
}
