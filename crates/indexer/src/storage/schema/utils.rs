mod schema;


impl schema::Document {
    pub fn new(doc_id: u64, url: String, path: String, title: String, content_length: u32, last_modified: u64) -> Self {
        Self {
            doc_id,
            url,
            path,
            title,
            content_length,
            last_modified,
        }
    }
    pub fn get_doc_id(&self) -> u64 {
        self.doc_id
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_content_length(&self) -> u32 {
        self.content_length
    }
    pub fn get_last_modified(&self) -> u64 {
        self.last_modified
    }
};


impl schema::Posting{
    pub fn new(doc_id: u64, term_frequency: u32) -> Self {
        Self {
            doc_id,
            term_frequency,
        }
    }
    pub fn get_doc_id(&self) -> u64 {
        self.doc_id
    }
    pub fn get_term_frequency(&self) -> u32 {
        self.term_frequency
    }
};


impl schema::Index{
    pub fn new() -> Self {
        Self {
            dictionary: HashMap::new(),
            documents: HashMap::new(),
        }
    }
    pub fn add_document(&mut self, document: schema::Document) {
        self.documents.insert(document.get_doc_id(), document);
    }
    pub fn add_posting(&mut self, term: String, posting: schema::Posting) {
        self.dictionary.entry(term).or_insert_with(Vec::new).push(posting);
    }
    pub fn get_document(&self, doc_id: u64) -> Option<&schema::Document> {
        self.documents.get(&doc_id)
    }
    pub fn get_postings(&self, term: &str) -> Option<&Vec<schema::Posting>> {
        self.dictionary.get(term)
    }
}