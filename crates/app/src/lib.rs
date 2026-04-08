pub mod config;
pub mod error;
pub mod runner;

use crate::config::load_app_config;
use crate::error::AppResult;
use crate::runner::SearchEngineApp;

pub fn load_environment() {
    let _ = dotenvy::dotenv();
}

pub async fn run() -> AppResult<()> {
    load_environment();
    SearchEngineApp::new(load_app_config()).run().await
}
