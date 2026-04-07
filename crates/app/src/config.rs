use std::env;
use std::path::PathBuf;

const DEFAULT_INDEX_PATH: &str = "index.json";
const DEFAULT_SEED_FILE: &str = "seeds.txt";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub index_path: PathBuf,
    pub seed_file: PathBuf,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            index_path: env::var_os("PERNOX_INDEX_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DEFAULT_INDEX_PATH)),
            seed_file: env::var_os("PERNOX_SEED_FILE")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DEFAULT_SEED_FILE)),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
