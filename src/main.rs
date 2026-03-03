use search_engine::crawler::{dns, seed::loader};
use search_engine::{Index, engine};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index_path = "index.json";
    let mut tf_index: Index = engine::load_index(index_path)?;

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
                if let Err(e) = engine::index_directory(folder, &mut tf_index) {
                    eprintln!("Error indexing: {}", e);
                } else {
                    engine::save_index(index_path, &tf_index)?;
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
                    "Found '{}' in {} files, total occurrences (content length): {}",
                    keyword, file_count, total_count
                );
            }
            "quit" | "exit" => break,

            "serve" | "start" => {
                println!("Serving the server ");
                let port = Option::None;
                search_engine::server::http::start_server(port);
            }
            "seeds" => {
                let seed = loader::consume_seeds_from_file();
                println!("{:?}", seed);
                let seed_manager = loader::create_seed();
                println!("{:?}", seed_manager);
                for ip in seed_manager.iter() {
                    let address = ip.host().unwrap().to_string();
                    let dns_record = dns::resolve_ip_to_dns(&address);
                    println!("{:?}", dns_record);
                }
            }
            _ => {
                println!(
                    "Unknown command: {}. Available: add, search, seeds, serve, quit",
                    parts[0]
                );
            }
        }
    }

    Ok(())
}
