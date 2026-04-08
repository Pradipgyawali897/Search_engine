pub mod config;
pub mod error;
pub mod models;
pub mod pool;
pub mod postgres;
pub mod queries;
pub mod repository;
pub mod schema;
pub mod schema_sql;

pub use config::PostgresConfig;
pub use error::{DbError, DbResult};
pub use models::{
    CrawlStatus, CrawlTarget, DiscoveredLink, Document, DocumentContent, DocumentTerm,
    LinkCategory, Posting, Term, UrlParts, parse_canonical_url,
};
pub use pool::{apply_schema, connect, connect_and_initialize};
pub use repository::{SearchEngineRepository, StoredDocument};
