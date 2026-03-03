use std::collections::HashMap;
use std::path::PathBuf;

pub type TF = HashMap<String, usize>;

pub type Index = HashMap<PathBuf, TF>;

pub mod engine;
pub mod lexer;
pub mod server;
pub mod crawler;