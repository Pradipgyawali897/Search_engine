use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, TF>;

pub mod parser;
pub mod tokenizer;
pub mod storage;
pub mod indexing;

pub use parser::html::HtmlParser;
pub use parser::xml::XmlParser;
pub use parser::Parser;
pub use indexing::tf_counter::index_file;
