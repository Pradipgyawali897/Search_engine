use crate::frontier::manager::Frontier;
use std::fs;

pub fn consume_seeds_from_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap_or_else(|_err| String::new());

    content.lines().map(|line| line.to_string()).collect()
}

pub fn create_seed(seeds: Vec<String>) -> Frontier {
    let file_seed: Vec<&str> = seeds.iter().map(|s| s.as_str()).collect();
    Frontier::new(file_seed)
}
