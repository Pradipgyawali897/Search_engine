pub async fn get_html_content(domain:&str) -> Option<String> {
    let url = format!("https://{}/", domain);
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
   