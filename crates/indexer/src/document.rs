use crate::TF;

#[derive(Debug, Clone, Default)]
pub struct ParsedDocument {
    pub text: String,
    pub links: Vec<String>,
}

impl ParsedDocument {
    pub fn new(text: String) -> Self {
        Self {
            text,
            links: Vec::new(),
        }
    }

    pub fn with_links(mut self, links: Vec<String>) -> Self {
        self.links = links;
        self
    }
}

#[derive(Debug, Clone)]
pub struct IndexedDocument {
    pub parsed_document: ParsedDocument,
    pub term_frequency: TF,
}

impl IndexedDocument {
    pub fn new(parsed_document: ParsedDocument, term_frequency: TF) -> Self {
        Self {
            parsed_document,
            term_frequency,
        }
    }
}
