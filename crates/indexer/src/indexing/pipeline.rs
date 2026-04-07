use crate::TF;
use crate::config::RuntimePaths;
use crate::discovery::process_links;
use crate::indexing::analyzer::build_term_frequency;
use crate::parser::Parser;

pub async fn index_file(
    domain: &str,
    parser: impl Parser,
    paths: &RuntimePaths,
) -> Result<TF, Box<dyn std::error::Error>> {
    let document = parser.parse(domain).await?;
    process_links(paths, &document.links);
    Ok(build_term_frequency(&document.text))
}
