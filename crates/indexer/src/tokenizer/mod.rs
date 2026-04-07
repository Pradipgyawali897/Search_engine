mod normalizer;
mod scanner;

pub use scanner::Tokenizer;

use crate::discovery::{canonicalize_url, is_valid_url, sanitize_url_candidate};

pub fn tokenize(content: &str) -> Vec<String> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokenizer = Tokenizer::new(&chars);
    let mut tokens = Vec::new();
    while let Some(token_chars) = tokenizer.next_token() {
        let raw_token: String = token_chars.iter().collect();
        tokens.extend(normalizer::normalize_token(&raw_token));
    }
    tokens
}

pub fn extract_urls(content: &str) -> Vec<String> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokenizer = Tokenizer::new(&chars);
    let mut urls = Vec::new();
    while let Some(token_chars) = tokenizer.next_token() {
        let token_str: String = token_chars.iter().collect();
        if let Some(url) = sanitize_url_candidate(&token_str)
            .filter(|candidate| is_valid_url(candidate))
            .and_then(|candidate| canonicalize_url(&candidate))
        {
            urls.push(url);
        }
    }
    urls
}
