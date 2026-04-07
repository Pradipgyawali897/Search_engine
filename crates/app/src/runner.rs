use crate::config::AppConfig;
use crate::error::AppResult;
use indexer::load_visited_urls;
use indexer::storage::engine as storage_engine;
use indexer::{HtmlParser, Index};
use std::path::Path;
use std::thread;

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
        let preload_handle = thread::spawn(load_visited_urls);
        let seeds = self.load_seeds();

        if seeds.is_empty() {
            println!(
                "No seeds found in {}. Please add some URLs to it.",
                self.config.seed_file.display()
            );
            preload_handle
                .join()
                .map_err(|_| "visited URL preloading thread panicked")?;
            return Ok(());
        }

        println!(
            "Found {} seeds. Fetching robots.txt for each...",
            seeds.len()
        );
        preload_handle
            .join()
            .map_err(|_| "visited URL preloading thread panicked")?;

        for (index, seed) in seeds.iter().enumerate() {
            self.process_seed(index + 1, seeds.len(), seed, &mut tf_index)
                .await;
        }

        self.save_index(&tf_index)?;
        println!("\nExecution completed.");
        Ok(())
    }

    fn load_index(&self) -> AppResult<Index> {
        storage_engine::load_index(self.path_as_str(&self.config.index_path)?).map_err(Into::into)
    }

    fn load_seeds(&self) -> Vec<String> {
        spyder::consume_seeds_from_file(&self.config.seed_file.to_string_lossy())
    }

    async fn process_seed(&self, current: usize, total: usize, seed: &str, tf_index: &mut Index) {
        println!("\n[{}/{}] Processing: {}", current, total, seed);

        match spyder::get_robot_content(seed).await {
            Some(_) => {
                println!("Indexing and discovering links: {}", seed);
            }
            None => {
                println!("No robots.txt found or error occurred for {}", seed);
                return;
            }
        }

        match indexer::index_file(seed, HtmlParser).await {
            Ok(tf) => {
                println!("Successfully indexed! Found {} unique tokens.", tf.len());
                tf_index.insert(seed.into(), tf);
            }
            Err(error) => {
                eprintln!("Failed to index {}: {}", seed, error);
            }
        }
    }

    fn save_index(&self, tf_index: &Index) -> AppResult<()> {
        println!("Saving index to {}...", self.config.index_path.display());
        storage_engine::save_index(self.path_as_str(&self.config.index_path)?, tf_index)
            .map_err(Into::into)
    }

    fn path_as_str<'a>(&self, path: &'a Path) -> AppResult<&'a str> {
        path.to_str()
            .ok_or_else(|| format!("path contains non-UTF-8 characters: {}", path.display()).into())
    }
}
