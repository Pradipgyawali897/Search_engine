use spyder::Frontier;
use spyder::dns::resolver;
use url::Url;

#[test]
fn test_dns_resolve_localhost() {
    let result = resolver::resolve_ip_to_dns("localhost");
    assert!(result.is_ok());
}

#[test]
fn test_frontier_new_parses_valid_seeds() {
    let manager = Frontier::new(vec!["http://google.com", "http://example.com"]);
    let urls: Vec<&Url> = manager.iter().collect();
    assert_eq!(urls.len(), 2);
}

#[test]
fn test_frontier_deduplication() {
    let mut manager = Frontier::new(vec!["http://google.com"]);
    manager.add_url("http://google.com");
    let urls: Vec<&Url> = manager.iter().collect();
    assert_eq!(urls.len(), 1);
}

#[test]
fn test_frontier_accepts_bare_hostname_seeds() {
    let manager = Frontier::new(vec!["example.com"]);
    let urls: Vec<&Url> = manager.iter().collect();
    assert_eq!(urls.len(), 1);
    assert_eq!(urls[0].as_str(), "https://example.com/");
}

#[test]
fn test_frontier_normalizes_duplicate_urls() {
    let mut manager = Frontier::new(vec!["https://example.com"]);
    assert!(!manager.add_url("https://EXAMPLE.com:443#section"));
    assert_eq!(manager.len(), 1);
}
