use std::collections::{HashSet, VecDeque};
use url::Url;

#[derive(Debug)]
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
                self.frontier.push_back(Url::parse(&normalized).unwrap());
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Url> {
        self.frontier.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parses_valid_seeds() {
        let manager = SeedManager::new(vec!["http://google.com", "http://example.com"]);
        let urls: Vec<&Url> = manager.iter().collect();
        assert_eq!(urls.len(), 2);
    }

    #[test]
    fn new_skips_invalid_seeds() {
        let manager = SeedManager::new(vec!["http://google.com", "not a url"]);
        let urls: Vec<&Url> = manager.iter().collect();
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn new_deduplicates_seeds() {
        let manager = SeedManager::new(vec!["http://google.com", "http://google.com"]);
        let urls: Vec<&Url> = manager.iter().collect();
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn next_url_returns_in_order() {
        let mut manager = SeedManager::new(vec!["http://first.com", "http://second.com"]);
        let first = manager.next_url().unwrap();
        assert_eq!(first.host_str(), Some("first.com"));
        let second = manager.next_url().unwrap();
        assert_eq!(second.host_str(), Some("second.com"));
        assert!(manager.next_url().is_none());
    }

    #[test]
    fn add_url_skips_duplicates() {
        let mut manager = SeedManager::new(vec!["http://google.com"]);
        manager.add_url("http://google.com");
        let urls: Vec<&Url> = manager.iter().collect();
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn add_url_adds_new() {
        let mut manager = SeedManager::new(vec!["http://google.com"]);
        manager.add_url("http://example.com");
        let urls: Vec<&Url> = manager.iter().collect();
        assert_eq!(urls.len(), 2);
    }
}
