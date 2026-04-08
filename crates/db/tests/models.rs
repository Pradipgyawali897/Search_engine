use db::{DiscoveredLink, Document, DocumentTerm, LinkCategory, Term};

#[test]
fn discovered_link_serializes_like_existing_json_output() {
    let link = DiscoveredLink::new("https://example.com/page", LinkCategory::Junk, 42).unwrap();
    let json = serde_json::to_string(&link).unwrap();

    assert_eq!(
        json,
        r#"{"url":"https://example.com/page","category":"junk","timestamp":42}"#
    );
}

#[test]
fn document_new_extracts_url_parts() {
    let document = Document::new("https://example.com/docs").unwrap();

    assert_eq!(document.canonical_url, "https://example.com/docs");
    assert_eq!(document.scheme, "https");
    assert_eq!(document.host, "example.com");
    assert_eq!(document.path, "/docs");
}

#[test]
fn term_rejects_empty_values() {
    let error = Term::new("   ").unwrap_err();
    assert!(error.to_string().contains("term cannot be empty"));
}

#[test]
fn document_term_rejects_non_positive_frequency() {
    let error = DocumentTerm::new(1, 2, 0).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("term frequency must be greater than zero")
    );
}
