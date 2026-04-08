use crate::config::AbWebConfig;
use crate::error::AppError;
use crate::models::ContentRecord;
use scraper::{Html, Selector};
use serde_json::Value;

#[derive(Clone)]
pub struct AbSourceClient {
    config: AbWebConfig,
    client: reqwest::Client,
}

impl AbSourceClient {
    pub fn new(config: AbWebConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(config.request_timeout)
            .build()
            .expect("reqwest client should build");

        Self { config, client }
    }

    pub async fn fetch_content(&self) -> Result<ContentRecord, AppError> {
        let mut request = self.client.get(&self.config.source_url);

        if let Some(api_key) = &self.config.api_key {
            let header_value = match &self.config.api_key_prefix {
                Some(prefix) if !prefix.trim().is_empty() => {
                    format!("{} {}", prefix.trim(), api_key)
                }
                _ => api_key.to_string(),
            };
            request = request.header(&self.config.api_key_header, header_value);
        }

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(AppError::upstream(format!(
                "AB returned status {}",
                response.status()
            )));
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());
        let payload = response.text().await?;

        build_content_record(
            &self.config.source_name,
            &self.config.source_url,
            content_type,
            payload,
        )
    }
}

fn build_content_record(
    source_name: &str,
    source_url: &str,
    content_type: Option<String>,
    payload: String,
) -> Result<ContentRecord, AppError> {
    let normalized_content_type = content_type
        .clone()
        .unwrap_or_else(|| "text/plain".to_string());

    if looks_like_json(content_type.as_deref(), &payload) {
        return build_from_json(source_name, source_url, normalized_content_type, payload);
    }

    if looks_like_html(content_type.as_deref(), &payload) {
        return build_from_html(source_name, source_url, normalized_content_type, payload);
    }

    ContentRecord::new(
        source_name,
        source_url,
        None,
        "",
        payload.clone(),
        Some(payload),
        Some(normalized_content_type),
        None,
    )
    .map_err(|error| AppError::upstream(error.to_string()))
}

fn build_from_json(
    source_name: &str,
    source_url: &str,
    content_type: String,
    payload: String,
) -> Result<ContentRecord, AppError> {
    let json: Value = serde_json::from_str(&payload)?;
    let title = first_string(&json, &["title", "name", "headline", "subject"]);
    let summary =
        first_string(&json, &["summary", "description", "excerpt", "subtitle"]).unwrap_or_default();
    let body = first_string(&json, &["body", "content", "text", "description"])
        .unwrap_or_else(|| serde_json::to_string_pretty(&json).unwrap_or(payload.clone()));

    ContentRecord::new(
        source_name,
        source_url,
        title,
        summary,
        body,
        Some(payload),
        Some(content_type),
        None,
    )
    .map_err(|error| AppError::upstream(error.to_string()))
}

fn build_from_html(
    source_name: &str,
    source_url: &str,
    content_type: String,
    payload: String,
) -> Result<ContentRecord, AppError> {
    let document = Html::parse_document(&payload);
    let title = select_text(&document, "title");
    let summary = select_meta_description(&document).unwrap_or_default();
    let body = select_body_text(&document);

    ContentRecord::new(
        source_name,
        source_url,
        title,
        summary,
        body,
        Some(payload),
        Some(content_type),
        None,
    )
    .map_err(|error| AppError::upstream(error.to_string()))
}

fn looks_like_json(content_type: Option<&str>, payload: &str) -> bool {
    content_type.is_some_and(|value| value.contains("json"))
        || payload.trim_start().starts_with('{')
        || payload.trim_start().starts_with('[')
}

fn looks_like_html(content_type: Option<&str>, payload: &str) -> bool {
    content_type.is_some_and(|value| value.contains("html"))
        || payload.contains("<html")
        || payload.contains("<body")
}

fn first_string(value: &Value, keys: &[&str]) -> Option<String> {
    if let Some(object) = value.as_object() {
        for key in keys {
            if let Some(string) = object.get(*key).and_then(Value::as_str) {
                let normalized = string.trim().to_string();
                if !normalized.is_empty() {
                    return Some(normalized);
                }
            }
        }
    }

    None
}

fn select_text(document: &Html, selector: &str) -> Option<String> {
    let selector = Selector::parse(selector).ok()?;
    let text = document
        .select(&selector)
        .next()
        .map(|element| element.text().collect::<Vec<_>>().join(" "))?;
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    (!normalized.is_empty()).then_some(normalized)
}

fn select_meta_description(document: &Html) -> Option<String> {
    let selector =
        Selector::parse("meta[name=\"description\"], meta[property=\"og:description\"]").ok()?;
    document
        .select(&selector)
        .find_map(|element| element.value().attr("content"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn select_body_text(document: &Html) -> String {
    let selector = Selector::parse("h1, h2, h3, p, li, blockquote").ok();
    let mut segments = Vec::new();

    if let Some(selector) = selector {
        for element in document.select(&selector) {
            let text = element.text().collect::<Vec<_>>().join(" ");
            let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
            if !normalized.is_empty() {
                segments.push(normalized);
            }
        }
    }

    if segments.is_empty() {
        return document.root_element().text().collect::<Vec<_>>().join(" ");
    }

    segments.join("\n\n")
}
