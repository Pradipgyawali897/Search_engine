use crate::config::AppConfig;
use crate::error::AppResult;
use futures::stream::{self, StreamExt};
use indexer::config::RuntimePaths;
use indexer::load_visited_urls;
use indexer::storage::engine as storage_engine;
use indexer::{HtmlParser, Index, TF};
use std::path::Path;

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

pub struct SearchEngineApp {
    config: AppConfig,
}

impl SearchEngineApp {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> AppResult<()> {
        println!("Pernox Kernel Execution...");

        let mut tf_index = self.load_index()?;
        load_visited_urls(&self.config.paths);
        let seeds = self.load_seeds();

        if seeds.is_empty() {
            println!(
                "No seeds found in {}. Please add some URLs to it.",
                self.config.paths.seed_path.display()
            );
            return Ok(());
        }

        println!(
            "Found {} seeds. Fetching robots.txt for each...",
            seeds.len()
        );

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
            tf_index.insert(result.0.into(), result.1);
        }

        self.save_index(&tf_index)?;
        println!("\nExecution completed.");
        Ok(())
    }

    fn load_index(&self) -> AppResult<Index> {
        storage_engine::load_index(self.path_as_str(&self.config.paths.index_path)?)
            .map_err(Into::into)
    }

    fn load_seeds(&self) -> Vec<String> {
        spyder::consume_seeds_from_file(&self.config.paths.seed_path.to_string_lossy())
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
}
