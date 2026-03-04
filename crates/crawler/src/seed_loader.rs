use std::fs;
use crate::frontier::Frontier;

pub fn consume_seeds_from_file() -> Vec<String> {
    let content = fs::read_to_string("seed.txt").unwrap_or_else(|_err| {
        String::new()
    });

    content.lines().map(|line| line.to_string()).collect()
}

pub fn create_seed() -> Frontier {
    let string_file_seed = consume_seeds_from_file();
    let file_seed: Vec<&str> = string_file_seed.iter().map(|s| s.as_str()).collect();
    Frontier::new(file_seed)
}
