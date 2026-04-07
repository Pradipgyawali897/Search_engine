use crate::storage::schema::schema::DiscoveredLink;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

lazy_static! {
    static ref VISITED_URLS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkCategory {
    Visitable,
    Junk,
}

pub fn process_links(links: &[String]) {
    for link in links {
        process_link(link);
    }
}

pub fn process_link(raw_url: &str) {
    let Some(url) = canonicalize_url(raw_url) else {
        return;
    };

    match classify_link(&url) {
        LinkCategory::Visitable => record_visitable(&url),
        LinkCategory::Junk => record_junk(&url),
    }
}

pub fn load_visited_urls() {
    let file = match File::open("visitable_urls.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("[discovery] No visitable_urls.txt found");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut visited = VISITED_URLS.lock().unwrap();
    let mut loaded = 0usize;

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };

        let Some(url) = canonicalize_url(&line) else {
            continue;
        };

        if visited.insert(create_hash(&url)) {
            loaded += 1;
        }
    }

    println!("[discovery] Pre-loaded {} visited URLs", loaded);
}

pub fn classify_link(url: &str) -> LinkCategory {
    if is_junk(url) {
        LinkCategory::Junk
    } else {
        LinkCategory::Visitable
    }
}

pub fn is_junk(url: &str) -> bool {
    let lower_url = url.to_lowercase();

    let junk_extensions = [
        ".jpg", ".jpeg", ".png", ".gif", ".svg", ".bmp", ".webp", ".mp4", ".webm", ".ogg", ".avi",
        ".mov", ".mp3", ".wav", ".flac", ".pdf", ".doc", ".docx", ".zip", ".tar", ".gz", ".css",
        ".js", ".json", ".xml",
    ];

    if junk_extensions
        .iter()
        .any(|extension| lower_url.ends_with(extension))
    {
        return true;
    }

    let junk_patterns = [
        "facebook.com",
        "twitter.com",
        "instagram.com",
        "linkedin.com",
        "google-analytics.com",
        "googletagmanager.com",
        "doubleclick.net",
        "ads.google.com",
        "adservice.google.com",
    ];

    junk_patterns
        .iter()
        .any(|pattern| lower_url.contains(pattern))
}

pub fn sanitize_url_candidate(raw: &str) -> Option<String> {
    let trimmed = raw.trim_matches(|c: char| {
        c.is_whitespace() || matches!(c, '"' | '\'' | '<' | '>' | '(' | '[' | '{')
    });
    let sanitized = trimmed.trim_end_matches(|c: char| {
        matches!(c, '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}')
    });

    if sanitized.is_empty() {
        None
    } else {
        Some(sanitized.to_string())
    }
}

pub fn is_valid_url(raw: &str) -> bool {
    canonicalize_url(raw).is_some()
}

pub fn canonicalize_url(raw: &str) -> Option<String> {
    let candidate = sanitize_url_candidate(raw)?;
    let mut url = parse_url(&candidate)?;

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

    Some(url.to_string())
}

fn parse_url(candidate: &str) -> Option<Url> {
    if let Ok(url) = Url::parse(candidate) {
        return Some(url);
    }

    let looks_like_hostname = candidate.starts_with("www.")
        || (candidate.contains('.') && candidate.chars().any(|ch| ch.is_alphabetic()));
    if candidate.contains("://") || !looks_like_hostname {
        return None;
    }

    Url::parse(&format!("https://{}", candidate)).ok()
}

fn record_visitable(url: &str) {
    let hash = create_hash(url);
    {
        let mut visited = VISITED_URLS.lock().unwrap();
        if !visited.insert(hash) {
            return;
        }
    }

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("visitable_urls.txt")
    {
        let _ = writeln!(file, "{}", url);
    }
}

fn record_junk(url: &str) {
    let discovered_link = DiscoveredLink {
        url: url.to_string(),
        category: "junk".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("junk_urls.json")
    {
        if let Ok(json) = serde_json::to_string(&discovered_link) {
            let _ = writeln!(file, "{}", json);
        }
    }
}

fn create_hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}
