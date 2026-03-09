use crate::tokenizer::Tokenizer;
use crate::TF;
use crate::parser::html::Parser;

pub async fn index_file(domain: &str, parser: impl Parser) -> Result<TF, Box<dyn std::error::Error>> {
    let content_str = parser.parse(domain).await?;
    let content: Vec<char> = content_str.chars().collect();
    let mut tf = TF::new();
    let mut tokenizer = Tokenizer::new(&content);
    while let Some(token_chars) = tokenizer.next_token() {
        let token: String = token_chars.iter().collect::<String>().to_lowercase();
        *tf.entry(token).or_insert(0) += 1;
    }
    Ok(tf)
}
