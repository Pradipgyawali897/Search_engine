use search_engine::indexer::{self, storage};
use search_engine::parser::xml::XmlParser;
use search_engine::Index;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index_path = "index.json";
    let mut tf_index: Index = storage::load_index(index_path)?;
    let parser = XmlParser;

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
                if parts.len() < 2 {
                    println!("Usage: add <folder>");
                    continue;
                }
                let folder = parts[1];
                if let Err(e) = indexer::index_directory(folder, &mut tf_index, &parser) {
                    eprintln!("Error indexing: {}", e);
                } else {
                    storage::save_index(index_path, &tf_index)?;
                    println!("Indexed {} files.", tf_index.len());
                }
            }
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: search <keyword>");
                    continue;
                }
                let keyword = parts[1].to_lowercase();
                let mut total_count = 0;
                let mut file_count = 0;

                for (_path, tf) in &tf_index {
                    if let Some(&count) = tf.get(&keyword) {
                        total_count += count;
                        file_count += 1;
                    }
                }

                println!(
                    "Found '{}' in {} files, total occurrences: {}",
                    keyword, file_count, total_count
                );
            }
            "serve" | "start" => {
                println!("Starting the server...");
                let port = Option::None;
                search_engine::server::http::start_server(port);
            }
            "quit" | "exit" => break,
            _ => {
                println!(
                    "Unknown command: {}. Available: add, search, serve, quit",
                    parts[0]
                );
            }
        }
    }

    Ok(())
}
