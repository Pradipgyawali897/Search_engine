use std::path::PathBuf;

pub const DEFAULT_INDEX_PATH: &str = "index.json";
pub const DEFAULT_SEED_PATH: &str = "seeds.txt";
pub const DEFAULT_VISITABLE_URLS_PATH: &str = "visitable_urls.txt";
pub const DEFAULT_JUNK_URLS_PATH: &str = "junk_urls.json";

#[derive(Debug, Clone)]
pub struct RuntimePaths {
    pub index_path: PathBuf,
    pub seed_path: PathBuf,
    pub visitable_urls_path: PathBuf,
    pub junk_urls_path: PathBuf,
}

impl RuntimePaths {
    pub fn new(
        index_path: PathBuf,
        seed_path: PathBuf,
        visitable_urls_path: PathBuf,
        junk_urls_path: PathBuf,
    ) -> Self {
        Self {
            index_path,
            seed_path,
            visitable_urls_path,
            junk_urls_path,
        }
    }
}
