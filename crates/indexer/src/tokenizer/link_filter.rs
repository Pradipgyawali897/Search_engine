pub enum LinkCategory {
    Visitable,
    Junk,
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

    for ext in junk_extensions {
        if lower_url.ends_with(ext) {
            return true;
        }
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

    for pattern in junk_patterns {
        if lower_url.contains(pattern) {
            return true;
        }
    }

    false
}
