use ab_web::models::{
    ContentDetail, ContentDetailEnvelope, ContentListEnvelope, ContentListItem, summarize_text,
};

#[test]
fn content_list_envelope_serializes_expected_shape() {
    let envelope = ContentListEnvelope::fresh(vec![sample_list_item()], 20);
    let json = serde_json::to_value(&envelope).unwrap();

    assert_eq!(json["meta"]["state"], "fresh");
    assert_eq!(json["meta"]["limit"], 20);
    assert_eq!(json["items"][0]["canonical_url"], "https://example.com/article");
    assert_eq!(json["items"][0]["summary"], "A concise preview of the stored article body.");
}

#[test]
fn content_detail_envelope_serializes_expected_shape() {
    let envelope = ContentDetailEnvelope::fresh(sample_detail());
    let json = serde_json::to_value(&envelope).unwrap();

    assert_eq!(json["meta"]["document_id"], 42);
    assert_eq!(json["content"]["title"], "Article title");
    assert_eq!(json["content"]["plain_text"], "Full article body with more text for reading.");
}

#[test]
fn summarize_text_truncates_long_values() {
    let summary = summarize_text(
        "This is a long body of text that should be shortened once the configured limit is crossed.",
        24,
    );

    assert_eq!(summary, "This is a long body of t...");
}

fn sample_list_item() -> ContentListItem {
    ContentListItem {
        id: 42,
        crawl_target_id: Some(12),
        canonical_url: "https://example.com/article".to_string(),
        host: "example.com".to_string(),
        path: "/article".to_string(),
        title: Some("Article title".to_string()),
        summary: "A concise preview of the stored article body.".to_string(),
        content_type: Some("text/html".to_string()),
        http_status: Some(200),
        language: Some("en".to_string()),
        content_length: 44,
        extracted_links_count: 2,
        fetched_at: 1_710_000_000,
        indexed_at: Some(1_710_000_060),
    }
}

fn sample_detail() -> ContentDetail {
    let list_item = sample_list_item();
    ContentDetail {
        id: list_item.id,
        crawl_target_id: list_item.crawl_target_id,
        canonical_url: list_item.canonical_url,
        host: list_item.host,
        path: list_item.path,
        title: list_item.title,
        summary: list_item.summary,
        plain_text: "Full article body with more text for reading.".to_string(),
        raw_html: Some("<p>Full article body with more text for reading.</p>".to_string()),
        content_type: list_item.content_type,
        http_status: list_item.http_status,
        language: list_item.language,
        content_length: list_item.content_length,
        extracted_links_count: list_item.extracted_links_count,
        fetched_at: list_item.fetched_at,
        indexed_at: list_item.indexed_at,
    }
}
