pub mod html;
pub mod xml;

#[allow(async_fn_in_trait)]
pub trait Parser {
    async fn parse(&self, domain: &str) -> Result<String, Box<dyn std::error::Error>>;
}
