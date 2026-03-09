use scraper::Html;
use std::path::PathBuf;
use std::fs;

pub trait Parser {
    fn parse(&self, path: &PathBuf) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct HtmlParser;

impl Parser for HtmlParser {
    fn parse(&self, path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let document = Html::parse_document(&content);
        
        // Simple text extraction from the parsed HTML document
        let mut text = String::new();
        for node in document.root_element().descendants() {
            if let Some(t) = node.value().as_text() {
                text.push_str(t);
                text.push(' ');
            }
        }
        Ok(text)
    }
}
