use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;

pub type Index = HashMap<PathBuf, TF>;

pub mod parser;
pub mod indexer;
pub mod lexer;
pub mod crawler;
pub mod server;