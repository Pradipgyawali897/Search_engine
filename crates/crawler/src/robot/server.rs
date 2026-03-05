use reqwest;

pub async fn check_robot(domain: &str) -> Result<Option<String>, reqwest::Error> {
    let url = format!("https://{}/robots.txt", domain);
    println!("Checking robots.txt for {}", url);

    let resp = reqwest::get(&url).await?;
    println!("Response: {:#?}", resp);

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let text = resp.text().await?;
    Ok(Some(text))
}
