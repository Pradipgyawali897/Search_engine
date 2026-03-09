use crate::parser::Parser;
use xml::reader::{EventReader, XmlEvent};
use crawler::parser::server::fetch_html::get_html_content;

pub struct XmlParser;

impl Parser for XmlParser {
    async fn parse(&self, domain: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = get_html_content(domain).await.ok_or("Failed to fetch XML content")?;
        let parser = EventReader::from_str(&content);
        let mut text = String::new();

        for e in parser {
            match e {
                Ok(XmlEvent::Characters(data)) => {
                    text.push_str(&data);
                    text.push(' ');
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
                _ => {}
            }
        }
        Ok(text.trim().to_string())
    }
}
