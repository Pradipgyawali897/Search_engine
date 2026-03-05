use reqwest;

pub async fn check_robot(domain: &str) -> Result<Option<String>, reqwest::Error> {
    let mut clean_domain = domain.trim();
    if clean_domain.starts_with("http://") {
        clean_domain = &clean_domain[7..];
    } else if clean_domain.starts_with("https://") {
        clean_domain = &clean_domain[8..];
    }
    
    // Extract only the host part (before the first slash)
    let host = clean_domain.split('/').next().unwrap_or(clean_domain);
    
    let url = format!("https://{}/robots.txt", host);
    println!("Checking robots.txt for {}", url);

    let resp = reqwest::get(&url).await?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let text = resp.text().await?;
    Ok(Some(text))
}
