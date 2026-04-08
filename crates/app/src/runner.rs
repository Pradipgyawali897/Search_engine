use crate::config::AppConfig;
use crate::error::AppResult;
use db::{DiscoveredLink, SearchEngineRepository};
use futures::stream::{FuturesUnordered, StreamExt};
use indexer::config::RuntimePaths;
use indexer::discovery::{LinkCategory, canonicalize_url, classify_link, process_links};
use indexer::load_visited_urls;
use indexer::storage::engine as storage_engine;
use indexer::{HtmlParser, Index, IndexedDocument, TF, index_document};
use std::collections::HashSet;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

struct CrawlTaskResult<T> {
    output: Option<T>,
    discovered_links: Vec<String>,
}

impl<T> CrawlTaskResult<T> {
    fn success(output: T, discovered_links: Vec<String>) -> Self {
        Self {
            output: Some(output),
            discovered_links,
        }
    }

    fn failed() -> Self {
        Self {
            output: None,
            discovered_links: Vec::new(),
        }
    }
}

async fn process_seed(
    paths: RuntimePaths,
    current: usize,
    seed: String,
) -> CrawlTaskResult<(String, TF)> {
    println!("\n[{}] Processing: {}", current, seed);

    match spyder::get_robot_content(&seed).await {
        Some(_) => {
            println!("Indexing and discovering links: {}", seed);
        }
        None => {
            println!(
                "No robots.txt found or it could not be fetched for {}. Proceeding from the seed URL.",
                seed
            );
        }
    }

    match index_document(&seed, HtmlParser).await {
        Ok(indexed_document) => {
            let discovered_links = indexed_document.parsed_document.links;
            let term_frequency = indexed_document.term_frequency;
            process_links(&paths, &discovered_links);
            println!(
                "Successfully indexed! Found {} unique tokens and {} discovered links.",
                term_frequency.len(),
                discovered_links.len()
            );
            CrawlTaskResult::success((seed, term_frequency), discovered_links)
        }
        Err(error) => {
            eprintln!("Failed to index {}: {}", seed, error);
            CrawlTaskResult::failed()
        }
    }
}

async fn process_seed_with_db(
    repository: SearchEngineRepository,
    current: usize,
    seed: String,
) -> CrawlTaskResult<()> {
    println!("\n[{}] Processing: {}", current, seed);

    match spyder::get_robot_content(&seed).await {
        Some(_) => {
            println!("Indexing and persisting to PostgreSQL: {}", seed);
        }
        None => {
            println!(
                "No robots.txt found or it could not be fetched for {}. Proceeding from the seed URL.",
                seed
            );
        }
    }

    match index_document(&seed, HtmlParser).await {
        Ok(indexed_document) => {
            let discovered_links = indexed_document.parsed_document.links.clone();

            if let Err(error) = persist_indexed_seed(&repository, &seed, &indexed_document).await {
                eprintln!("Failed to persist {}: {}", seed, error);
                return CrawlTaskResult::failed();
            }

            CrawlTaskResult::success((), discovered_links)
        }
        Err(error) => {
            eprintln!("Failed to index {}: {}", seed, error);
            CrawlTaskResult::failed()
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
        println!("Loading seeds from {}...", seed_path.display());
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
            "Found {} seeds. Starting crawl from the seed frontier.",
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
        let paths = self.config.paths.clone();

        self.crawl_frontier(
            seeds,
            move |current, seed| {
                let paths = paths.clone();
                async move { process_seed(paths, current, seed).await }
            },
            |(url, term_frequency)| {
                tf_index.insert(PathBuf::from(url), term_frequency);
                Ok(())
            },
        )
        .await?;

        self.save_index(&tf_index)?;
        Ok(())
    }

    async fn run_with_database(
        &self,
        seeds: Vec<String>,
        repository: SearchEngineRepository,
    ) -> AppResult<()> {
        self.crawl_frontier(
            seeds,
            move |current, seed| {
                let repository = repository.clone();
                async move { process_seed_with_db(repository, current, seed).await }
            },
            |_| Ok(()),
        )
        .await?;

        Ok(())
    }

    async fn crawl_frontier<T, F, Fut, H>(
        &self,
        seeds: Vec<String>,
        mut process: F,
        mut handle_output: H,
    ) -> AppResult<()>
    where
        F: FnMut(usize, String) -> Fut,
        Fut: Future<Output = CrawlTaskResult<T>>,
        H: FnMut(T) -> AppResult<()>,
    {
        let mut frontier = spyder::create_seed(seeds);
        let mut in_flight = FuturesUnordered::new();
        let mut scheduled = 0usize;
        let mut completed = 0usize;
        let concurrency = self.config.concurrency.max(1);
        let crawl_limit = self.config.max_crawl_urls.unwrap_or(usize::MAX);
        let unlimited = self.config.max_crawl_urls.is_none();

        if unlimited {
            println!("Crawl limit disabled. The fetch will continue until the frontier is empty.");
        } else {
            println!("Crawl limit set to {} URLs.", crawl_limit);
        }

        loop {
            while in_flight.len() < concurrency && scheduled < crawl_limit {
                let Some(next_url) = frontier.next_url() else {
                    break;
                };

                scheduled += 1;
                in_flight.push(process(scheduled, next_url.to_string()));
            }

            if in_flight.is_empty() {
                break;
            }

            let result = in_flight
                .next()
                .await
                .expect("crawl queue should contain in-flight work");
            completed += 1;

            let newly_queued = enqueue_discovered_links(&mut frontier, &result.discovered_links);
            if !result.discovered_links.is_empty() {
                println!(
                    "[{}] Queued {} new visitable URLs from {} discovered links. Frontier size: {}",
                    completed,
                    newly_queued,
                    result.discovered_links.len(),
                    frontier.len()
                );
            }

            if let Some(value) = result.output {
                handle_output(value)?;
            }
        }

        if !unlimited && scheduled >= crawl_limit && (!frontier.is_empty() || !in_flight.is_empty())
        {
            println!(
                "Reached crawl limit of {} URLs. Increase PERNOX_MAX_CRAWL_URLS or set it to 0 for an unbounded crawl.",
                crawl_limit
            );
        }

        println!(
            "Crawl finished after scheduling {} URLs and completing {} fetches.",
            scheduled, completed
        );
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

fn enqueue_discovered_links(frontier: &mut spyder::Frontier, links: &[String]) -> usize {
    links.iter().fold(0usize, |queued, raw_link| {
        let Some(canonical_url) = canonicalize_url(raw_link) else {
            return queued;
        };

        if matches!(classify_link(&canonical_url), LinkCategory::Visitable)
            && frontier.add_url(&canonical_url)
        {
            queued + 1
        } else {
            queued
        }
    })
}

async fn persist_indexed_seed(
    repository: &SearchEngineRepository,
    seed: &str,
    indexed_document: &IndexedDocument,
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
    use super::{enqueue_discovered_links, resolve_seed_path};
    use spyder::create_seed;
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

    #[test]
    fn enqueue_discovered_links_adds_only_new_visitable_urls() {
        let mut frontier = create_seed(vec!["https://example.com".to_string()]);
        let discovered_links = vec![
            "https://example.com/about".to_string(),
            "https://example.com/about#team".to_string(),
            "https://cdn.example.com/app.js".to_string(),
        ];

        assert_eq!(
            enqueue_discovered_links(&mut frontier, &discovered_links),
            1
        );
        assert_eq!(frontier.len(), 2);
    }

    fn unique_temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("pernox-app-tests-{}-{nanos}", std::process::id()))
    }
}
