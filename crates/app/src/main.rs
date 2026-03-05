use indexer::parser::html::HtmlParser;
use indexer::storage::engine as storage_engine;
use indexer::{self, Index};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index_path = "index.json";
    let mut tf_index: Index = storage_engine::load_index(index_path)?;
    let parser = HtmlParser;

    println!("Search Engine Hierarchical App Running...");

    loop {
        print!("Search Engine > ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "add" => {
                let folder = parts.get(1).unwrap_or(&"");
                if folder.is_empty() {
                    println!("Usage: add <folder>");
                    continue;
                }
                if let Err(e) = indexer::index_directory(folder, &mut tf_index, &parser) {
                    eprintln!("Error indexing: {}", e);
                } else {
                    storage_engine::save_index(index_path, &tf_index)?;
                    println!("Indexed {} files.", tf_index.len());
                }
            }
            "search" => {
                let keyword = parts.get(1).unwrap_or(&"");
                if keyword.is_empty() {
                    println!("Usage: search <keyword>");
                    continue;
                }
                let (file_count, total_count) = searcher::find_occurrences(keyword, &tf_index);
                println!(
                    "Found '{}' in {} files, total occurrences: {}",
                    keyword, file_count, total_count
                );
            }
            "quit" | "exit" => break,
            "robot" => {
                let domain = parts.get(1).unwrap_or(&"");
                if domain.is_empty() {
                    println!("Usage: robot <domain>");
                    continue;
                }
                let robot = crawler::get_robot_content(domain).await;
                println!("Robot: {:#?}", robot);
            }
            "seed" => {
                let name = parts.get(1).unwrap_or(&"");
                if name.is_empty() {
                    println!("Usage: seed <url_or_filepath>");
                    continue;
                }
                if name.ends_with(".txt") {
                    match crawler::consume_seeds_from_file(name) {
                        Ok(seeds) => println!("Loaded {} seeds from file.", seeds.len()),
                        Err(e) => eprintln!("Error loading seeds: {}", e),
                    }
                } else {
                    let seed = crawler::create_seed(name);
                    println!("Created seed: {:#?}", seed);
                }
            }
            _ => println!(
                "Unknown command: {}. Available: add, search, quit, robot, seed",
                parts[0]
            ),
        }
    }
    Ok(())
}
