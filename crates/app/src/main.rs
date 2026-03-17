use indexer::parser::html::HtmlParser;
use indexer::storage::engine as storage_engine;
use indexer::{self, Index};
use std::thread;
use indexer::tokenizer::load_visited_urls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index_path = "index.json";
    let mut tf_index: Index = storage_engine::load_index(index_path)?;
    let _parser = HtmlParser;

    println!("Pernox Kernel Execution...");

    let handel=thread::spawn(|| {
        load_visited_urls();
    });
    let seed_file = "seeds.txt";
    let seeds = spyder::consume_seeds_from_file(seed_file);

    if seeds.is_empty() {
        println!("No seeds found in {}. Please add some URLs to it.", seed_file);
        return Ok(());
    }

    println!("Found {} seeds. Fetching robots.txt for each...", seeds.len());
    handel.join().unwrap();
    for (i, seed) in seeds.iter().enumerate() {
        println!("\n[{}/{}] Processing: {}", i + 1, seeds.len(), seed);
        let robot = spyder::get_robot_content(seed).await;
        if robot.is_none() {
            println!("No robots.txt found or error occurred for {}", seed);
        } else {
            println!("Indexing and discovering links: {}", seed);
            match indexer::index_file(seed, HtmlParser).await {
                Ok(tf) => {
                    println!("Successfully indexed! Found {} unique tokens.", tf.len());
                    tf_index.insert(std::path::PathBuf::from(seed), tf);
                }
                Err(err) => {
                    eprintln!("Failed to index {}: {}", seed, err);
                }
            }
        }
    }

    println!("Saving index to {}...", index_path);
    storage_engine::save_index(index_path, &tf_index)?;

    println!("\nExecution completed.");
    Ok(())
}
