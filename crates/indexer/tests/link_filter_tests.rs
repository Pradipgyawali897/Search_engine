use indexer::discovery::{LinkCategory, classify_link, is_junk};

#[test]
fn test_is_junk_extensions() {
    assert!(is_junk("https://example.com/image.jpg"));
    assert!(is_junk("https://example.com/script.js"));
    assert!(is_junk("https://example.com/data.json"));
    assert!(!is_junk("https://example.com/page"));
    assert!(!is_junk("https://example.com/about.html"));
}

#[test]
fn test_is_junk_patterns() {
    assert!(is_junk("https://www.facebook.com/user"));
    assert!(is_junk("https://google-analytics.com/collect"));
    assert!(!is_junk("https://my-search-engine.com"));
}

#[test]
fn test_classify_link() {
    match classify_link("https://example.com/image.jpg") {
        LinkCategory::Junk => (),
        _ => panic!("Should be junk"),
    }
    match classify_link("https://example.com/page") {
        LinkCategory::Visitable => (),
        _ => panic!("Should be visitable"),
    }
}
