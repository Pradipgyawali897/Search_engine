use std::fs;

use crate::crawler::seed_manager::manager::SeedManager;

fn consume_seeds_from_file() -> Vec<String> {
    let content = fs::read_to_string("seed.txt").unwrap_or_else(|err| {
        eprintln!("error reading the seeds {err}");
        String::new()
    });

    let frountier: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    frountier
}

pub fn create_seed() {
    let string_file_seed = consume_seeds_from_file();
    let file_seed = string_file_seed.iter().map(|s| s.as_str()).collect();
    let seed = SeedManager::new(file_seed);
}
