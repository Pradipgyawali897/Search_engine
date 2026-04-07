pub async fn get_html_content(domain: &str) -> Option<String> {
    let url = if domain.starts_with("http://") || domain.starts_with("https://") {
        domain.to_string()
    } else {
        format!("https://{}/", domain)
    };
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Failed to fetch URL: {} | error: {}", url, err);
            return None;
        }
    };

    match resp.text().await {
        Ok(t) => Some(t),
        Err(err) => {
            eprintln!("Failed to fetch URL: {} | error: {}", url, err);
            None
        }
    }
}
