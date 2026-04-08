use crate::models::ContentListEnvelope;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ContentCache {
    ttl: Duration,
    state: Arc<Mutex<Option<CacheEntry>>>,
}

#[derive(Clone)]
struct CacheEntry {
    collection: ContentListEnvelope,
    inserted_at: Instant,
}

impl ContentCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            state: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get(&self, limit: u32) -> Option<ContentListEnvelope> {
        let mut guard = self.state.lock().ok()?;
        let entry = guard.as_ref()?;

        if entry.inserted_at.elapsed() >= self.ttl {
            *guard = None;
            return None;
        }

        if entry.collection.meta.limit != limit {
            return None;
        }

        Some(entry.collection.clone())
    }

    pub async fn set(&self, collection: ContentListEnvelope) {
        if let Ok(mut guard) = self.state.lock() {
            *guard = Some(CacheEntry {
                collection,
                inserted_at: Instant::now(),
            });
        }
    }
}
