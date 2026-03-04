use std::fs::File;
use std::path::PathBuf;
use xml::reader::{EventReader, XmlEvent};

pub trait Parser {
    fn parse(&self, path: &PathBuf) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct HtmlParser;

impl Parser for HtmlParser {
    fn parse(&self, file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let er = EventReader::new(file);
        let mut content = String::new();
        for event in er {
            match event? {
                XmlEvent::Characters(text) => content.push_str(&text),
                _ => {}
            }
        }
        Ok(content)
    }
}
