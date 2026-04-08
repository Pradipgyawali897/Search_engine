use crate::schema::DEFAULT_POSTGRES_SCHEMA;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
    pub schema: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
}

impl PostgresConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            schema: DEFAULT_POSTGRES_SCHEMA.to_string(),
            max_connections: 10,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(5),
        }
    }
}
