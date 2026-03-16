use url::Url;
use std::fs::OpenOptions;
use std::io::Write;
use spyder::normalize_url;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn save_url(url: &str, category: LinkCategory) {
    let (filename, category_str) = match category {
        LinkCategory::Visitable => ("visitable_urls.json", "visitable"),
        LinkCategory::Junk => ("junk_urls.json", "junk"),
    };

    let content_to_save = if category_str == "visitable" {
        normalize_url(url).unwrap_or_else(|| url.to_string())
    } else {
        url.to_string()
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
