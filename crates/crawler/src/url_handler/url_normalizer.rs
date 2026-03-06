use url::Url;

pub fn normalize_url(input: &str) -> Option<String> {
    let mut url = match Url::parse(input) {
        Ok(u) => u,
        Err(err) => {
            eprintln!("Failed to parse url: {} | error: {}", input, err);
            return None;
        }
    };

    url.set_fragment(None);

    if let Some(host) = url.host_str() {
        let lower = host.to_lowercase();
        url.set_host(Some(&lower)).ok()?;
    }

    match (url.scheme(), url.port()) {
        ("http" | "https", Some(_val)) => {
            url.set_port(None).ok()?;
        }
        _ => {}
    }

    Some(url.to_string())
}
