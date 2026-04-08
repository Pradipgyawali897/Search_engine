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
            if let Some(parsed) = parse_frontier_url(seed) {
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

    pub fn add_url(&mut self, link: &str) -> bool {
        if let Some(parsed) = parse_frontier_url(link) {
            if self.visited.insert(parsed.clone()) {
                self.frontier.push_back(parsed);
                return true;
            }
        }

        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &Url> {
        self.frontier.iter()
    }

    pub fn len(&self) -> usize {
        self.frontier.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frontier.is_empty()
    }
}

fn parse_frontier_url(candidate: &str) -> Option<Url> {
    let candidate = candidate.trim();
    if candidate.is_empty() {
        return None;
    }

    let mut url = if let Ok(url) = Url::parse(candidate) {
        url
    } else {
        let looks_like_hostname = candidate.starts_with("www.")
            || (candidate.contains('.') && candidate.chars().any(|ch| ch.is_alphabetic()));
        if candidate.contains("://") || !looks_like_hostname {
            return None;
        }

        Url::parse(&format!("https://{}", candidate)).ok()?
    };

    url.set_fragment(None);

    if let Some(host) = url.host_str() {
        url.set_host(Some(&host.to_lowercase())).ok()?;
    }

    if matches!(
        (url.scheme(), url.port()),
        ("http", Some(80)) | ("https", Some(443))
    ) {
        url.set_port(None).ok()?;
    }

    if url.path().is_empty() {
        url.set_path("/");
    }

    Some(url)
}
