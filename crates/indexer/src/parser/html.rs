use scraper::{Html, Selector};
use spyder::parser::server::fetch_html::get_html_content;
use url::Url;

use crate::ParsedDocument;
use crate::parser::Parser;

pub struct HtmlParser;

impl Parser for HtmlParser {
    async fn parse(&self, domain: &str) -> Result<ParsedDocument, Box<dyn std::error::Error>> {
        let content = get_html_content(domain)
            .await
            .ok_or("Failed to fetch HTML content")?;
        Ok(parse_html_document(&content, domain))
    }
}

pub fn parse_html_document(content: &str, domain: &str) -> ParsedDocument {
    let document = Html::parse_document(content);
    let base_url = create_base_url(domain);

    let mut links = Vec::new();
    let selector = Selector::parse("a[href]").unwrap();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(link) = resolve_link(base_url.as_ref(), href) {
                links.push(link);
            }
        }
    }

    let mut segments = Vec::new();

    if let Some(title_selector) = Selector::parse("title").ok() {
        for element in document.select(&title_selector) {
            push_clean_text(&mut segments, &element.text().collect::<Vec<_>>().join(" "));
        }
    }

    if let Some(meta_selector) =
        Selector::parse("meta[name=\"description\"], meta[property=\"og:description\"]").ok()
    {
        for element in document.select(&meta_selector) {
            if let Some(description) = element.value().attr("content") {
                push_clean_text(&mut segments, description);
            }
        }
    }

    if let Some(content_selector) =
        Selector::parse("h1, h2, h3, h4, h5, h6, p, li, blockquote, pre, figcaption, td, th").ok()
    {
        for element in document.select(&content_selector) {
            push_clean_text(&mut segments, &element.text().collect::<Vec<_>>().join(" "));
        }
    }

    if segments.is_empty() {
        if let Some(body_selector) = Selector::parse("body").ok() {
            if let Some(body) = document.select(&body_selector).next() {
                push_clean_text(&mut segments, &body.text().collect::<Vec<_>>().join(" "));
            }
        }
    }

    ParsedDocument::new(segments.join(" ")).with_links(links)
}

fn create_base_url(domain: &str) -> Option<Url> {
    if let Ok(url) = Url::parse(domain) {
        return Some(url);
    }

    Url::parse(&format!("https://{}", domain)).ok()
}

fn resolve_link(base_url: Option<&Url>, href: &str) -> Option<String> {
    if href.starts_with('#') || href.starts_with("javascript:") || href.starts_with("mailto:") {
        return None;
    }

    if let Ok(url) = Url::parse(href) {
        return Some(url.to_string());
    }

    base_url.and_then(|base| base.join(href).ok().map(|url| url.to_string()))
}

fn push_clean_text(segments: &mut Vec<String>, text: &str) {
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if !cleaned.is_empty() {
        segments.push(cleaned);
    }
}
