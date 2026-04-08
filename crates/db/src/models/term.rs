use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Term {
    pub id: Option<i64>,
    pub term: String,
    pub document_frequency: i64,
}

impl Term {
    pub fn new(term: impl Into<String>) -> DbResult<Self> {
        let normalized = term.into().trim().to_string();
        if normalized.is_empty() {
            return Err(DbError::Validation("term cannot be empty".to_string()));
        }

        Ok(Self {
            id: None,
            term: normalized,
            document_frequency: 0,
        })
    }
}
