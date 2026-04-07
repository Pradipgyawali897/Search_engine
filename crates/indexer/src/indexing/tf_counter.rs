use crate::TF;
use crate::parser::Parser;
use crate::tokenizer::tokenize;

pub async fn index_file(
    domain: &str,
    parser: impl Parser,
) -> Result<TF, Box<dyn std::error::Error>> {
    let content_str = parser.parse(domain).await?;
    let mut tf = TF::new();

    for token in tokenize(&content_str) {
        *tf.entry(token).or_insert(0) += 1;
    }

    Ok(tf)
}
