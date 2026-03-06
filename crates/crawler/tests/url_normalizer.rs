use crawler::url_handler::url_normalizer::normalize_url;

#[test]
fn test_url_normalizer() {
    let url = "http://google.com";
    let normalized = normalize_url(url);
    assert_eq!(normalized, Some("google.com".to_string()));
}

#[test]
fn test_url_normalizer_with_port() {
    let url = "http://google.com:2040";
    let normalized = normalize_url(url);
    assert_eq!(normalized, Some("google.com".to_string()));
}

#[test]
fn test_url_normalizer_with_port_path() {
    let url = "http://google.com:2040/robot";
    let normalized = normalize_url(url);
    assert_eq!(normalized, Some("google.com".to_string()));
}

#[test]
fn test_url_normalizer_with_port_path_fragment() {
    let url = "http://google.com:2040/robot#123";
    let normalized = normalize_url(url);
    assert_eq!(normalized, Some("google.com".to_string()));
}
#[test]
fn handel_for_none() {
    let url = "";
    let normalized = normalize_url(url);
    assert_eq!(normalized, None);
}
