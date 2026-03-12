use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GENERATED: Mutex<HashSet<u32>> = Mutex::new(HashSet::new());
}

pub fn unique_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    loop {
        let n = rng.gen_range(u32::MIN..=u32::MAX);
        let mut set = GENERATED.lock().unwrap();
        if set.insert(n) {
            return n;
        }
    }
}