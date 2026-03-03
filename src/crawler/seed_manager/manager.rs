use std::collections::{VecDeque, HashSet};
use url::Url;

pub struct SeedManager {
    frontier: VecDeque<Url>,
    visited: HashSet<String>,
}

impl SeedManager {
    pub fn new(seeds: Vec<&str>) -> Self {
        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();

        for seed in seeds {
            if let Ok(parsed) = Url::parse(seed) {
                let normalized = parsed.to_string();
                if visited.insert(normalized.clone()) {
                    frontier.push_back(Url::parse(&normalized).unwrap());
                }
            }
        }

        Self { frontier, visited }
    }
    pub fn next_url(&mut self) -> Option<Url> {
        self.frontier.pop_front()
    }

     pub fn add_url(&mut self, link: &str) {
        if let Ok(parsed) = Url::parse(link) {
            let normalized = parsed.to_string();

            if self.visited.insert(normalized.clone()) {
                self.frontier.push_back(
                    Url::parse(&normalized).unwrap()
                );
            }
        }
    }
}