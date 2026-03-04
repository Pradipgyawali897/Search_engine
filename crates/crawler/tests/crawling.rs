use crawler::Frontier;

#[test]
fn test_integration_crawling_seed_management() {
    let mut manager = Frontier::new(vec!["http://example.com"]);
    manager.add_url("http://test.com");
    
    let first = manager.next_url().unwrap();
    assert_eq!(first.as_str(), "http://example.com/");
    
    let second = manager.next_url().unwrap();
    assert_eq!(second.as_str(), "http://test.com/");
}
