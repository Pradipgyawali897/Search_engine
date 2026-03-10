use url::Url;
use std::fs::OpenOptions;
use std::io::Write;

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

pub fn save_url(url: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("discovered_urls.txt")
    {
        let _ = writeln!(file, "{}", url);
    }
}
