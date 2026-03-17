use std::fs::File;
use std::io::{BufRead, BufReader};
use spyder::normalize_url;
use crate::globals::VISITED_URLS;

use super::utils::create_hash;

pub fn load_visited_urls() {
    let file = match File::open("visitable_urls.txt") {
        Ok(f) => f,
        Err(_) => {println!("[load_data] No visitable_urls.txt found"); return;},
    };

    let reader = BufReader::new(file);
    let mut visited = VISITED_URLS.lock().unwrap();
    let mut loaded = 0usize;

    for line in reader.lines() {
        let line = match line {
            Ok(l) if !l.trim().is_empty() => l,
            _ => continue,
        };
        let url = normalize_url(&line).unwrap_or(line);
        let hash = create_hash(&url);
        if !visited.contains(&hash) {
            visited.insert(hash);
            loaded += 1;
        }
        else{
            println!("[load_data] URL already visited: {}", url);
        }
    }

    println!("[load_data] Pre-loaded {} visited URL hashes from visitable_urls.txt", loaded);
}
