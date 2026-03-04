use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, TF>;

pub mod tokenizer;
pub mod html_parser;
pub mod storage_engine;
pub mod tf_counter;

pub use tf_counter::{index_directory, index_file};
