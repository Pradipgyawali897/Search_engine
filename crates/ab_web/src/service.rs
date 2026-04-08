use crate::cache::ContentCache;
use crate::config::AbWebConfig;
use crate::error::AppError;
use crate::models::{ContentDetailEnvelope, ContentListEnvelope};
use crate::storage::ContentStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<AppService>,
}

pub struct AppService {
    cache: ContentCache,
    store: ContentStore,
    default_limit: u32,
    max_limit: u32,
}

impl AppService {
    pub fn new(config: AbWebConfig) -> Result<Self, AppError> {
        Ok(Self {
            cache: ContentCache::new(config.cache_ttl),
            store: ContentStore::new(config.database_url, config.database_schema)?,
            default_limit: config.default_limit.max(1),
            max_limit: config.max_limit.max(1),
        })
    }

    pub fn state(self: &Arc<Self>) -> AppState {
        AppState {
            service: Arc::clone(self),
        }
    }

    pub async fn load_content(
        &self,
        force_refresh: bool,
        limit: u32,
    ) -> Result<ContentListEnvelope, AppError> {
        let limit = self.normalize_limit(limit);
        if !force_refresh {
            if let Some(collection) = self.cache.get(limit).await {
                return Ok(collection.cached());
            }
        }

        self.refresh_content_with_limit(limit).await
    }

    pub async fn refresh_content(&self) -> Result<ContentListEnvelope, AppError> {
        self.refresh_content_with_limit(self.default_limit).await
    }

    pub async fn load_document(&self, document_id: i64) -> Result<ContentDetailEnvelope, AppError> {
        let content = self
            .store
            .document_detail(document_id)
            .await?
            .ok_or_else(|| AppError::not_found(format!("document {document_id} was not found")))?;

        Ok(ContentDetailEnvelope::fresh(content))
    }

    async fn refresh_content_with_limit(&self, limit: u32) -> Result<ContentListEnvelope, AppError> {
        let collection = ContentListEnvelope::fresh(self.store.latest_documents(limit).await?, limit);
        self.cache.set(collection.clone()).await;
        Ok(collection)
    }

    fn normalize_limit(&self, limit: u32) -> u32 {
        if limit == 0 {
            return self.default_limit;
        }

        limit.min(self.max_limit)
    }
}
