pub mod config;
pub mod error;
pub mod runner;

use crate::config::load_app_config;
use crate::error::AppResult;
use crate::runner::SearchEngineApp;

pub fn load_environment() {
    if let Ok(path) = dotenvy::dotenv() {
        if let Some(parent) = path.parent() {
            unsafe {
                std::env::set_var("PERNOX_APP_BASE_DIR", parent);
            }
        }
    }
}

pub async fn run() -> AppResult<()> {
    load_environment();
    SearchEngineApp::new(load_app_config()).run().await
}
