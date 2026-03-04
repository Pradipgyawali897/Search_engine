use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, TF>;

pub mod lexer;
pub mod parser;
pub mod storage;
pub mod engine;

pub use engine::{index_directory, index_file};
