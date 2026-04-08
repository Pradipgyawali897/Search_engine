pub mod cache;
pub mod config;
pub mod error;
pub mod http;
pub mod models;
pub mod scheduler;
pub mod service;
pub mod storage;

use crate::config::AbWebConfig;
use crate::error::AppError;
use crate::http::router;
use crate::scheduler::spawn_refresh_job;
use crate::service::AppService;
use axum::serve;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn run() -> Result<(), AppError> {
    let _ = dotenvy::dotenv();
    let config = AbWebConfig::from_env()?;
    let service = Arc::new(AppService::new(config.clone())?);
    let router = router(service.state());
    let listener = TcpListener::bind(&config.bind_addr).await?;

    eprintln!(
        "[ab_web] scraped-content dashboard listening on http://{}",
        config.bind_addr
    );

    let refresh_job = spawn_refresh_job(Arc::clone(&service), config.refresh_interval);
    let server_result = serve(listener, router.into_make_service()).await;
    refresh_job.abort();

    server_result.map_err(AppError::from)
}
