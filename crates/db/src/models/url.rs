use crate::error::{DbError, DbResult};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlParts {
    pub canonical_url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
}

pub fn parse_canonical_url(raw_url: &str) -> DbResult<UrlParts> {
    let parsed = Url::parse(raw_url).map_err(|error| {
        DbError::Validation(format!("invalid canonical url '{raw_url}': {error}"))
    })?;
    let host = parsed.host_str().ok_or_else(|| {
        DbError::Validation(format!("canonical url '{raw_url}' must include a host"))
    })?;

    let path = if parsed.path().is_empty() {
        "/".to_string()
    } else {
        parsed.path().to_string()
    };

    Ok(UrlParts {
        canonical_url: parsed.to_string(),
        scheme: parsed.scheme().to_string(),
        host: host.to_string(),
        path,
    })
}
