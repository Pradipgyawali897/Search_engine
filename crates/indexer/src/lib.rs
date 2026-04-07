use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, TF>;

pub mod globals;
pub mod indexing;
pub mod parser;
pub mod storage;
pub mod tokenizer;

pub use indexing::tf_counter::index_file;
pub use parser::Parser;
pub use parser::html::HtmlParser;
pub use parser::xml::XmlParser;
