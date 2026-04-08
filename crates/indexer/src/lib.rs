use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, TF>;

pub mod config;
pub mod discovery;
pub mod document;
pub mod indexing;
pub mod parser;
pub mod storage;
pub mod tokenizer;

pub use discovery::load_visited_urls;
pub use document::{IndexedDocument, ParsedDocument};
pub use indexing::{index_document, index_file};
pub use parser::Parser;
pub use parser::html::{HtmlParser, parse_html_document};
pub use parser::xml::{XmlParser, parse_xml_document};
