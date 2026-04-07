mod core;
pub mod link_filter;
pub mod load_data;
pub mod utils;

pub use core::Tokenizer;
pub use load_data::load_visited_urls;

pub fn tokenize(content: &str) -> Vec<String> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokenizer = Tokenizer::new(&chars);
    let mut tokens = Vec::new();
    while let Some(token_chars) = tokenizer.next_token() {
        let raw_token: String = token_chars.iter().collect();
        tokens.extend(utils::normalize_token(&raw_token));
    }
    tokens
}

pub fn extract_urls(content: &str) -> Vec<String> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokenizer = Tokenizer::new(&chars);
    let mut urls = Vec::new();
    while let Some(token_chars) = tokenizer.next_token() {
        let token_str: String = token_chars.iter().collect();
        if let Some(url) =
            utils::sanitize_url_candidate(&token_str).filter(|url| utils::is_valid_url(url))
        {
            urls.push(url);
        }
    }
    urls
}
