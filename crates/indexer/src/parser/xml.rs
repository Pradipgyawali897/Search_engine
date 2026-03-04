use std::fs::File;
use std::path::PathBuf;
use xml::reader::{EventReader, XmlEvent};
use super::Parser;

pub struct XmlParser;

impl Parser for XmlParser {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_xml_extracts_text() {
        let dir = std::env::temp_dir().join("indexer_xml_parser_test");
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.xml");

        let mut file = File::create(&file_path).unwrap();
        write!(file, "<root><title>Hello</title><body>World</body></root>").unwrap();

        let parser = XmlParser;
        let result = parser.parse(&file_path).unwrap();
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
