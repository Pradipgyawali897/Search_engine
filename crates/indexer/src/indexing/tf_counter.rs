use crate::tokenizer::Tokenizer;
use crate::TF;
use crate::parser::Parser;

pub async fn index_file(domain: &str, parser: impl Parser) -> Result<TF, Box<dyn std::error::Error>> {
    let content_str = parser.parse(domain).await?;
    let content: Vec<char> = content_str.chars().collect();
    
    let mut tf = TF::new();
    let mut tokenizer = Tokenizer::new(&content);
    
    while let Some(token_chars) = tokenizer.next_token() {
        let mut token = String::with_capacity(token_chars.len());
        for &c in token_chars {
            token.push(c.to_ascii_lowercase());
        }
        *tf.entry(token).or_insert(0) += 1;
    }
    
    Ok(tf)
}
