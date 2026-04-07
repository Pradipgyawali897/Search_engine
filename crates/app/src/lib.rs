pub mod config;
pub mod error;
pub mod runner;

use crate::config::load_app_config;
use crate::error::AppResult;
use crate::runner::SearchEngineApp;

pub async fn run() -> AppResult<()> {
    SearchEngineApp::new(load_app_config()).run().await
}
