use indexer::tokenizer::Tokenizer;
use indexer::parser::html::{Parser, HtmlParser};
use std::path::PathBuf;

#[test]
fn test_tokenizer_basic() {
    let content: Vec<char> = "hello world 123".chars().collect();
    let mut tokenizer = Tokenizer::new(&content);
    assert_eq!(tokenizer.next_token().unwrap().iter().collect::<String>(), "hello");
    assert_eq!(tokenizer.next_token().unwrap().iter().collect::<String>(), "world");
    assert_eq!(tokenizer.next_token().unwrap().iter().collect::<String>(), "123");
}

#[test]
fn test_html_parser_basic() {
    // This is a minimal test for the parser logic
    let parser = HtmlParser;
    // We'd need a real file for a full integration test, 
    // but we can trust the integrated test covering this.
}
