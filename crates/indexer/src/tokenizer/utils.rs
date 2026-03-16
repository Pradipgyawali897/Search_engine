use url::Url;
use std::fs::OpenOptions;
use std::io::Write;

use super::link_filter::LinkCategory;

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
    let filename = match category {
        LinkCategory::Visitable => "visitable_urls.txt",
        LinkCategory::Junk => "junk_urls.txt",
    };

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        let _ = writeln!(file, "{}", url);
    }
}
