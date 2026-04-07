use crate::ParsedDocument;

pub mod html;
pub mod xml;

#[allow(async_fn_in_trait)]
pub trait Parser {
    async fn parse(&self, domain: &str) -> Result<ParsedDocument, Box<dyn std::error::Error>>;
}
