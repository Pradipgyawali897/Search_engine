use std::collections::HashMap;

pub struct Document {
    pub doc_id: u64,
    pub url: String,
    pub path: String,
    pub title: String,
    pub content_length: u32,
    pub last_modified: u64,
}

pub struct Posting {
    pub doc_id: u64,
    pub term_frequency: u32,
}

pub struct Index {
    pub dictionary: HashMap<String, Vec<Posting>>,
    pub documents: HashMap<u64, Document>,
}

