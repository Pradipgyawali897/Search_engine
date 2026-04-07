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
