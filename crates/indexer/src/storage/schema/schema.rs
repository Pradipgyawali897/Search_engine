use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub doc_id: u64,
    pub url: String,
    pub path: String,
    pub title: String,
    pub content_length: u32,
    pub last_modified: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Posting {
    pub doc_id: u64,
    pub term_frequency: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub dictionary: HashMap<String, Vec<Posting>>,
    pub documents: HashMap<u64, Document>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveredLink {
    pub url: String,
    pub category: String,
    pub timestamp: u64,
}
