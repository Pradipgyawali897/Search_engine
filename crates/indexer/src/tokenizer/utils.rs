use url::Url;
use std::fs::OpenOptions;
use std::io::Write;
use spyder::normalize_url;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::globals::VISITED_URLS;

use super::link_filter::LinkCategory;
use crate::storage::schema::schema::DiscoveredLink;


pub fn is_valid_url(s: &str) -> bool {
    let has_prefix = s.starts_with("http://") || s.starts_with("https://") || s.starts_with("www.");
    if !has_prefix {
        return false;
    }
    
    if s.starts_with("www.") {
        Url::parse(&format!("https://{}", s)).is_ok()
    } else {
        Url::parse(s).is_ok()
    }
}

pub fn create_hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn save_url(url: &str, category: LinkCategory) {
    let category_str = match category {
        LinkCategory::Visitable => "visitable",
        LinkCategory::Junk => "junk",
    };

    let content_to_save = if category_str == "visitable" {
        let normalized = normalize_url(url).unwrap_or_else(|| url.to_string());
        
        let hash = create_hash(&normalized);
        let mut visited = VISITED_URLS.lock().unwrap();
        if !visited.insert(hash) {
            return;
        }
        normalized
    } else {
        url.to_string()
    };

    let filename = if category_str == "visitable" {
        "visitable_urls.json"
    } else {
        "junk_urls.json"
    };

    let discovered_link = DiscoveredLink {
        url: content_to_save,
        category: category_str.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        if let Ok(json) = serde_json::to_string(&discovered_link) {
            let _ = writeln!(file, "{}", json);
        }
    }
}
