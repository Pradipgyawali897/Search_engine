use std::collections::{HashSet, VecDeque};
use url::Url;

#[derive(Debug)]
pub struct Frontier {
    frontier: VecDeque<Url>,
    visited: HashSet<Url>,
}

impl Frontier {
    pub fn new(seeds: Vec<&str>) -> Self {
        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();

        for seed in seeds {
            if let Ok(parsed) = Url::parse(seed) {
                if visited.insert(parsed.clone()) {
                    frontier.push_back(parsed);
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
            if self.visited.insert(parsed.clone()) {
                self.frontier.push_back(parsed);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Url> {
        self.frontier.iter()
    }
}
