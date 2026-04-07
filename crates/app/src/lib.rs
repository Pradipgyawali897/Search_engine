pub mod config;
pub mod error;
pub mod runner;

use crate::config::AppConfig;
use crate::error::AppResult;
use crate::runner::SearchEngineApp;

pub async fn run() -> AppResult<()> {
    SearchEngineApp::new(AppConfig::default()).run().await
}
