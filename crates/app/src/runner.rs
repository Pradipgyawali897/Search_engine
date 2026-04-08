use crate::config::AppConfig;
use crate::error::AppResult;
use db::{DiscoveredLink, SearchEngineRepository};
use futures::stream::{self, StreamExt};
use indexer::config::RuntimePaths;
use indexer::discovery::{LinkCategory, canonicalize_url, classify_link};
use indexer::load_visited_urls;
use indexer::storage::engine as storage_engine;
use indexer::{HtmlParser, Index, IndexedDocument, TF, index_document};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

async fn process_seed(
    paths: RuntimePaths,
    current: usize,
    total: usize,
    seed: String,
) -> Option<(String, TF)> {
    println!("\n[{}/{}] Processing: {}", current, total, seed);

    match spyder::get_robot_content(&seed).await {
        Some(_) => {
            println!("Indexing and discovering links: {}", seed);
        }
        None => {
            println!("No robots.txt found or error occurred for {}", seed);
            return None;
        }
    }

    match indexer::index_file(&seed, HtmlParser, &paths).await {
        Ok(tf) => {
            println!("Successfully indexed! Found {} unique tokens.", tf.len());
            Some((seed, tf))
        }
        Err(error) => {
            eprintln!("Failed to index {}: {}", seed, error);
            None
        }
    }
}

async fn process_seed_with_db(
    repository: SearchEngineRepository,
    current: usize,
    total: usize,
    seed: String,
) -> Option<()> {
    println!("\n[{}/{}] Processing: {}", current, total, seed);

    match spyder::get_robot_content(&seed).await {
        Some(_) => {
            println!("Indexing and persisting to PostgreSQL: {}", seed);
        }
        None => {
            println!("No robots.txt found or error occurred for {}", seed);
            return None;
        }
    }

    match index_document(&seed, HtmlParser).await {
        Ok(indexed_document) => {
            if let Err(error) = persist_indexed_seed(&repository, &seed, indexed_document).await {
                eprintln!("Failed to persist {}: {}", seed, error);
                return None;
            }

            Some(())
        }
        Err(error) => {
            eprintln!("Failed to index {}: {}", seed, error);
            None
        }
    }
}

pub struct SearchEngineApp {
    config: AppConfig,
}

impl SearchEngineApp {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> AppResult<()> {
        println!("Pernox Kernel Execution...");
        let seed_path = self.resolve_seed_path();
        if seed_path != self.config.paths.seed_path {
            println!(
                "Configured seed file {} was not found. Using {} instead.",
                self.config.paths.seed_path.display(),
                seed_path.display()
            );
        }
        let seeds = self.load_seeds(&seed_path);

        if seeds.is_empty() {
            println!(
                "No seeds found in {}. Please add some URLs to it.",
                seed_path.display()
            );
            return Ok(());
        }

        println!(
            "Found {} seeds. Fetching robots.txt for each...",
            seeds.len()
        );

        match &self.config.database {
            Some(database_config) => {
                println!(
                    "PostgreSQL scrape mode enabled. Using schema '{}'.",
                    database_config.schema
                );
                let repository = SearchEngineRepository::initialize(database_config).await?;
                self.run_with_database(seeds, repository).await?;
            }
            None => {
                println!("File scrape mode enabled. Set DATABASE_URL to persist into PostgreSQL.");
                self.run_with_files(seeds).await?;
            }
        }

        println!("\nExecution completed.");
        Ok(())
    }

    async fn run_with_files(&self, seeds: Vec<String>) -> AppResult<()> {
        let mut tf_index = self.load_index()?;
        load_visited_urls(&self.config.paths);

        let total = seeds.len();
        let concurrency = self.config.concurrency;
        let paths = self.config.paths.clone();

        let results = stream::iter(seeds.into_iter().enumerate())
            .map(|(i, seed)| {
                let paths = paths.clone();
                async move { process_seed(paths, i + 1, total, seed).await }
            })
            .buffer_unordered(concurrency)
            .collect::<Vec<_>>()
            .await;

        for result in results.into_iter().flatten() {
            tf_index.insert(PathBuf::from(result.0), result.1);
        }

        self.save_index(&tf_index)?;
        Ok(())
    }

    async fn run_with_database(
        &self,
        seeds: Vec<String>,
        repository: SearchEngineRepository,
    ) -> AppResult<()> {
        let total = seeds.len();
        let concurrency = self.config.concurrency;

        stream::iter(seeds.into_iter().enumerate())
            .map(|(i, seed)| {
                let repository = repository.clone();
                async move { process_seed_with_db(repository, i + 1, total, seed).await }
            })
            .buffer_unordered(concurrency)
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }

    fn load_index(&self) -> AppResult<Index> {
        storage_engine::load_index(self.path_as_str(&self.config.paths.index_path)?)
            .map_err(Into::into)
    }

    fn load_seeds(&self, seed_path: &Path) -> Vec<String> {
        spyder::consume_seeds_from_file(&seed_path.to_string_lossy())
    }

    fn save_index(&self, tf_index: &Index) -> AppResult<()> {
        println!(
            "Saving index to {}...",
            self.config.paths.index_path.display()
        );
        storage_engine::save_index(self.path_as_str(&self.config.paths.index_path)?, tf_index)
            .map_err(Into::into)
    }

    fn path_as_str<'a>(&self, path: &'a Path) -> AppResult<&'a str> {
        path.to_str()
            .ok_or_else(|| format!("path contains non-UTF-8 characters: {}", path.display()).into())
    }

    fn resolve_seed_path(&self) -> PathBuf {
        resolve_seed_path(&self.config.paths.seed_path)
    }
}

async fn persist_indexed_seed(
    repository: &SearchEngineRepository,
    seed: &str,
    indexed_document: IndexedDocument,
) -> AppResult<()> {
    let stored_document = repository
        .store_indexed_document(
            seed,
            &indexed_document.parsed_document.text,
            &indexed_document.term_frequency,
            indexed_document.parsed_document.links.len(),
        )
        .await?;

    let discovered_links = build_discovered_links(&indexed_document.parsed_document.links);
    repository
        .record_discovered_links(Some(stored_document.document_id), &discovered_links)
        .await?;

    println!(
        "Persisted document {} with {} indexed terms and {} discovered links.",
        stored_document.document_id,
        stored_document.indexed_terms,
        discovered_links.len()
    );

    Ok(())
}

fn build_discovered_links(links: &[String]) -> Vec<DiscoveredLink> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let mut seen = HashSet::new();
    let mut discovered_links = Vec::new();

    for raw_link in links {
        let Some(canonical_url) = canonicalize_url(raw_link) else {
            continue;
        };

        if !seen.insert(canonical_url.clone()) {
            continue;
        }

        let category = match classify_link(&canonical_url) {
            LinkCategory::Visitable => LinkCategory::Visitable,
            LinkCategory::Junk => LinkCategory::Junk,
        };

        if let Ok(discovered_link) = DiscoveredLink::new(canonical_url, category, timestamp) {
            discovered_links.push(discovered_link);
        }
    }

    discovered_links
}

fn resolve_seed_path(configured_path: &Path) -> PathBuf {
    if configured_path.is_file() {
        return configured_path.to_path_buf();
    }

    for candidate in alternate_seed_paths(configured_path) {
        if candidate.is_file() {
            return candidate;
        }
    }

    configured_path.to_path_buf()
}

fn alternate_seed_paths(configured_path: &Path) -> Vec<PathBuf> {
    let Some(file_name) = configured_path.file_name().and_then(|name| name.to_str()) else {
        return Vec::new();
    };

    let aliases: &[&str] = match file_name {
        "seeds.txt" => &["seed.txt"],
        "seed.txt" => &["seeds.txt"],
        _ => &[],
    };

    aliases
        .iter()
        .map(|alias| configured_path.with_file_name(alias))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::resolve_seed_path;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn resolve_seed_path_falls_back_to_seed_txt() {
        let temp_dir = unique_temp_dir();
        fs::create_dir_all(&temp_dir).unwrap();

        let configured = temp_dir.join("seeds.txt");
        let fallback = temp_dir.join("seed.txt");
        fs::write(&fallback, "https://example.com\n").unwrap();

        assert_eq!(resolve_seed_path(&configured), fallback);

        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn resolve_seed_path_keeps_existing_configured_file() {
        let temp_dir = unique_temp_dir();
        fs::create_dir_all(&temp_dir).unwrap();

        let configured = temp_dir.join("seeds.txt");
        fs::write(&configured, "https://example.com\n").unwrap();

        assert_eq!(resolve_seed_path(&configured), configured);

        fs::remove_dir_all(temp_dir).unwrap();
    }

    fn unique_temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("pernox-app-tests-{}-{nanos}", std::process::id()))
    }
}
