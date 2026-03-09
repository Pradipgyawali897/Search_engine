use spyder::dns::resolver;
use spyder::Frontier;
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

