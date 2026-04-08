use app::config::{load_app_config, load_database_config};
use std::sync::{Mutex, OnceLock};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn load_app_config_reads_database_environment() {
    let _guard = env_lock().lock().unwrap();
    clear_env();

    unsafe {
        std::env::set_var("PERNOX_DATABASE_URL", "postgres://localhost/pernox");
        std::env::set_var("PERNOX_DATABASE_SCHEMA", "pernox_test");
        std::env::set_var("PERNOX_DATABASE_MAX_CONNECTIONS", "17");
        std::env::set_var("PERNOX_CONCURRENCY", "4");
    }

    let config = load_app_config();
    let database = config.database.expect("database config should be present");

    assert_eq!(config.concurrency, 4);
    assert_eq!(database.database_url, "postgres://localhost/pernox");
    assert_eq!(database.schema, "pernox_test");
    assert_eq!(database.max_connections, 17);

    clear_env();
}

#[test]
fn load_database_config_ignores_empty_url() {
    let _guard = env_lock().lock().unwrap();
    clear_env();

    unsafe {
        std::env::set_var("PERNOX_DATABASE_URL", "   ");
    }

    assert!(load_database_config().is_none());
    clear_env();
}

fn clear_env() {
    for key in [
        "PERNOX_DATABASE_URL",
        "PERNOX_DATABASE_SCHEMA",
        "PERNOX_DATABASE_MAX_CONNECTIONS",
        "PERNOX_DATABASE_MIN_CONNECTIONS",
        "PERNOX_DATABASE_ACQUIRE_TIMEOUT_SECS",
        "PERNOX_CONCURRENCY",
    ] {
        unsafe {
            std::env::remove_var(key);
        }
    }
}
