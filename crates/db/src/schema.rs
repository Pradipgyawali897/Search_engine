pub use crate::models::{
    CrawlStatus, CrawlTarget, DiscoveredLink, Document, DocumentContent, DocumentTerm,
    LinkCategory, Posting, Term, UrlParts, parse_canonical_url,
};
pub use crate::schema_sql::postgres_schema_statements;

use crate::error::{DbError, DbResult};

pub const DEFAULT_POSTGRES_SCHEMA: &str = "search_engine";

pub fn validate_schema_name(namespace: &str) -> DbResult<()> {
    let mut chars = namespace.chars();
    let Some(first) = chars.next() else {
        return Err(DbError::Validation(
            "postgres schema name cannot be empty".to_string(),
        ));
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(DbError::Validation(format!(
            "postgres schema name '{namespace}' must start with a letter or underscore"
        )));
    }

    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        return Err(DbError::Validation(format!(
            "postgres schema name '{namespace}' may only contain letters, numbers, and underscores"
        )));
    }

    Ok(())
}
