use std::path::PathBuf;

pub mod xml;

pub trait Parser {
    fn parse(&self, file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>>;
}
