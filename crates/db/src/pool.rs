use crate::config::PostgresConfig;
use crate::error::{DbError, DbResult};
use crate::schema_sql::postgres_schema_statements;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect(config: &PostgresConfig) -> DbResult<PgPool> {
    let database_url = config.database_url.trim();
    if database_url.is_empty() {
        return Err(DbError::Validation(
            "postgres connection string cannot be empty".to_string(),
        ));
    }

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.acquire_timeout)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn apply_schema(pool: &PgPool, schema: &str) -> DbResult<()> {
    for statement in postgres_schema_statements(schema)? {
        sqlx::query(&statement).execute(pool).await?;
    }

    Ok(())
}

pub async fn connect_and_initialize(config: &PostgresConfig) -> DbResult<PgPool> {
    let pool = connect(config).await?;
    apply_schema(&pool, &config.schema).await?;
    Ok(pool)
}
