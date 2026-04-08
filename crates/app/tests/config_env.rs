use app::config::{load_app_config, load_database_config};
use app::load_environment;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

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

#[test]
fn load_database_config_accepts_standard_database_url_env() {
    let _guard = env_lock().lock().unwrap();
    clear_env();

    unsafe {
        std::env::set_var("DATABASE_URL", "postgres://localhost/pernox_main");
        std::env::set_var("DATABASE_SCHEMA", "public_search");
        std::env::set_var("DATABASE_MAX_CONNECTIONS", "9");
    }

    let database = load_database_config().expect("database config should be present");

    assert_eq!(database.database_url, "postgres://localhost/pernox_main");
    assert_eq!(database.schema, "public_search");
    assert_eq!(database.max_connections, 9);

    clear_env();
}

#[test]
fn load_environment_reads_seed_file_from_dotenv() {
    let _guard = env_lock().lock().unwrap();
    clear_env();

    let original_dir = std::env::current_dir().unwrap();
    let temp_dir = unique_temp_dir();
    std::fs::create_dir_all(&temp_dir).unwrap();
    std::fs::write(
        temp_dir.join(".env"),
        "PERNOX_SEED_FILE=seed.txt \nPERNOX_CONCURRENCY=5\n",
    )
    .unwrap();

    std::env::set_current_dir(&temp_dir).unwrap();
    load_environment();

    let config = load_app_config();
    assert_eq!(config.paths.seed_path, temp_dir.join("seed.txt"));
    assert_eq!(config.concurrency, 5);

    std::env::set_current_dir(original_dir).unwrap();
    std::fs::remove_dir_all(temp_dir).unwrap();
    clear_env();
}

fn clear_env() {
    for key in [
        "DATABASE_URL",
        "DATABASE_SCHEMA",
        "DATABASE_MAX_CONNECTIONS",
        "DATABASE_MIN_CONNECTIONS",
        "DATABASE_ACQUIRE_TIMEOUT_SECS",
        "PERNOX_DATABASE_URL",
        "PERNOX_DATABASE_SCHEMA",
        "PERNOX_DATABASE_MAX_CONNECTIONS",
        "PERNOX_DATABASE_MIN_CONNECTIONS",
        "PERNOX_DATABASE_ACQUIRE_TIMEOUT_SECS",
        "PERNOX_CONCURRENCY",
        "PERNOX_SEED_FILE",
        "PERNOX_APP_BASE_DIR",
    ] {
        unsafe {
            std::env::remove_var(key);
        }
    }
}

fn unique_temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "pernox-dotenv-tests-{}-{nanos}",
        std::process::id()
    ))
}
