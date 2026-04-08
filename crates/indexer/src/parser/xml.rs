use crate::ParsedDocument;
use crate::parser::Parser;
use spyder::parser::server::fetch_html::get_html_content;
use xml::reader::{EventReader, XmlEvent};

pub struct XmlParser;

impl Parser for XmlParser {
    async fn parse(&self, domain: &str) -> Result<ParsedDocument, Box<dyn std::error::Error>> {
        let content = get_html_content(domain)
            .await
            .ok_or("Failed to fetch XML content")?;
        parse_xml_document(&content)
    }
}

pub fn parse_xml_document(content: &str) -> Result<ParsedDocument, Box<dyn std::error::Error>> {
    let parser = EventReader::from_str(content);
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
    Ok(ParsedDocument::new(text.trim().to_string()))
}
