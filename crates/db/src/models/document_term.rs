use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentTerm {
    pub document_id: i64,
    pub term_id: i64,
    pub term_frequency: i32,
}

impl DocumentTerm {
    pub fn new(document_id: i64, term_id: i64, term_frequency: i32) -> DbResult<Self> {
        if term_frequency <= 0 {
            return Err(DbError::Validation(
                "term frequency must be greater than zero".to_string(),
            ));
        }

        Ok(Self {
            document_id,
            term_id,
            term_frequency,
        })
    }
}
