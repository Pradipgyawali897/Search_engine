use scraper::{Html, Selector};
use spyder::parser::server::fetch_html::get_html_content;

use crate::parser::Parser;
use crate::tokenizer::link_filter::classify_link;
use crate::tokenizer::utils::save_url;

pub struct HtmlParser;

impl Parser for HtmlParser {
    async fn parse(&self, domain: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = get_html_content(domain)
            .await
            .ok_or("Failed to fetch HTML content")?;
        let document = Html::parse_document(&content);

        let selector = Selector::parse("a").unwrap();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                let category = classify_link(href);
                save_url(href, category);
            }
        }

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
