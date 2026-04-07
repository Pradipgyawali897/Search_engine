use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    pub static ref VISITED_URLS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}
