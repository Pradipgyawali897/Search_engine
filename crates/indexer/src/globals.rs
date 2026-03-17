use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VISITED_URLS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}
