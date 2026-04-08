use crate::TF;
use crate::config::RuntimePaths;
use crate::discovery::process_links;
use crate::document::IndexedDocument;
use crate::indexing::analyzer::build_term_frequency;
use crate::parser::Parser;

pub async fn index_document(
    domain: &str,
    parser: impl Parser,
) -> Result<IndexedDocument, Box<dyn std::error::Error>> {
    let parsed_document = parser.parse(domain).await?;
    let term_frequency = build_term_frequency(&parsed_document.text);
    Ok(IndexedDocument::new(parsed_document, term_frequency))
}

pub async fn index_file(
    domain: &str,
    parser: impl Parser,
    paths: &RuntimePaths,
) -> Result<TF, Box<dyn std::error::Error>> {
    let indexed_document = index_document(domain, parser).await?;
    process_links(paths, &indexed_document.parsed_document.links);
    Ok(indexed_document.term_frequency)
}
