use std::path::PathBuf;

pub trait Parser {
    fn parse(&self, path: &PathBuf) -> Result<String, Box<dyn std::error::Error>>;
}

pub mod xml;
