use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, TF>;

pub mod parser;
pub mod tokenizer;
pub mod storage;
pub mod indexing;

pub use indexing::tf_counter::{index_directory, index_file};
