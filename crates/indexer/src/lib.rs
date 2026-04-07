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
pub use document::ParsedDocument;
pub use indexing::index_file;
pub use parser::Parser;
pub use parser::html::HtmlParser;
pub use parser::xml::XmlParser;
