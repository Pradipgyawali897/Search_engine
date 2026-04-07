use indexer::tokenizer::Tokenizer;

#[test]
fn test_tokenizer_basic() {
    let content: Vec<char> = "hello world 123".chars().collect();
    let mut tokenizer = Tokenizer::new(&content);
    assert_eq!(
        tokenizer.next_token().unwrap().iter().collect::<String>(),
        "hello"
    );
    assert_eq!(
        tokenizer.next_token().unwrap().iter().collect::<String>(),
        "world"
    );
    assert_eq!(
        tokenizer.next_token().unwrap().iter().collect::<String>(),
        "123"
    );
}

#[test]
fn test_tokenize_functional() {
    let tokens = indexer::tokenizer::tokenize("hello world 123");
    assert_eq!(tokens, vec!["hello", "world", "123"]);
}

#[test]
fn test_tokenize_handles_web_text_noise() {
    let tokens =
        indexer::tokenizer::tokenize("State-of-the-art search, don't stop. Version 2.1,000!");
    assert_eq!(
        tokens,
        vec![
            "state-of-the-art",
            "stateoftheart",
            "state",
            "of",
            "the",
            "art",
            "search",
            "don't",
            "dont",
            "stop",
            "version",
            "2.1,000"
        ]
    );
}

#[test]
fn test_extract_urls_functional() {
    let urls = indexer::tokenizer::extract_urls("visit https://google.com and www.rust-lang.org");
    assert_eq!(
        urls,
        vec!["https://google.com/", "https://www.rust-lang.org/"]
    );
}

#[test]
fn test_tokenize_skips_urls_from_text_index() {
    let tokens = indexer::tokenizer::tokenize("visit https://google.com for docs");
    assert_eq!(tokens, vec!["visit", "for", "docs"]);
}

#[test]
fn test_extract_urls_trims_trailing_punctuation() {
    let urls =
        indexer::tokenizer::extract_urls("Docs: https://www.rust-lang.org/, and https://docs.rs).");
    assert_eq!(urls, vec!["https://www.rust-lang.org/", "https://docs.rs/"]);
}

#[test]
fn test_tokenizer_url() {
    let content: Vec<char> = "visit https://google.com for info".chars().collect();
    let mut tokenizer = Tokenizer::new(&content);
    assert_eq!(
        tokenizer.next_token().unwrap().iter().collect::<String>(),
        "visit"
    );
    assert_eq!(
        tokenizer.next_token().unwrap().iter().collect::<String>(),
        "https://google.com"
    );
    assert_eq!(
        tokenizer.next_token().unwrap().iter().collect::<String>(),
        "for"
    );
}
