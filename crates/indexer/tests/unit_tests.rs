use indexer::tokenizer::Tokenizer;
// use indexer::HtmlParser;

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
fn test_extract_urls_functional() {
    let urls = indexer::tokenizer::extract_urls("visit https://google.com and www.rust-lang.org");
    assert_eq!(urls, vec!["https://google.com", "www.rust-lang.org"]);
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

    // Check if the file was created
    assert!(std::path::Path::new("discovered_urls.txt").exists());
}
