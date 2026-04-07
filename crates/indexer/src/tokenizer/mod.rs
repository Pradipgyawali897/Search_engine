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
        tokens.push(token_chars.iter().collect());
    }
    tokens
}

pub fn extract_urls(content: &str) -> Vec<String> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokenizer = Tokenizer::new(&chars);
    let mut urls = Vec::new();
    while let Some(token_chars) = tokenizer.next_token() {
        let token_str: String = token_chars.iter().collect();
        if utils::is_valid_url(&token_str) {
            urls.push(token_str);
        }
    }
    urls
}
