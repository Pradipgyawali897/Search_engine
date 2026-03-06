use indexer::parser::html::HtmlParser;
use indexer::storage::engine as storage_engine;
use indexer::{self, Index};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index_path = "index.json";
    let _tf_index: Index = storage_engine::load_index(index_path)?;
    let _parser = HtmlParser;

    println!("Search Engine Single Flow Execution...");

    let seed_file = "seeds.txt";
    let seeds = crawler::consume_seeds_from_file(seed_file);

    if seeds.is_empty() {
        println!("No seeds found in {}. Please add some URLs to it.", seed_file);
        return Ok(());
    }

    println!("Found {} seeds. Fetching robots.txt for each...", seeds.len());

    for (i, seed) in seeds.iter().enumerate() {
        println!("\n[{}/{}] Processing: {}", i + 1, seeds.len(), seed);
        let robot = crawler::get_robot_content(seed).await;
        if robot.is_none() {
            println!("No robots.txt found or error occurred for {}", seed);
        }
        
    }

    println!("\nExecution completed.");
    Ok(())
}
