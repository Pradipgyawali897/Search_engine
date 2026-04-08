use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::{ServeDir, ServeFile};

pub fn router(state: crate::service::AppState) -> Router {
    Router::new()
        .route("/api/content", get(get_content))
        .route("/api/content/:id", get(get_document))
        .nest_service(
            "/",
            get_service(
                ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).not_found_service(
                    ServeFile::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/index.html")),
                ),
            ),
        )
        .with_state(state)
}

async fn get_content(
    State(state): State<crate::service::AppState>,
    Query(query): Query<ContentQuery>,
) -> Response {
    match state
        .service
        .load_content(query.refresh, query.limit.unwrap_or_default())
        .await
    {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => error.into_response(),
    }
}

async fn get_document(
    State(state): State<crate::service::AppState>,
    Path(document_id): Path<i64>,
) -> Response {
    match state.service.load_document(document_id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => error.into_response(),
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ContentQuery {
    #[serde(default)]
    refresh: bool,
    limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for crate::error::AppError {
    fn into_response(self) -> Response {
        let status = match self {
            crate::error::AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            crate::error::AppError::NotFound(_) => StatusCode::NOT_FOUND,
            crate::error::AppError::Io(_) | crate::error::AppError::Sqlx(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (
            status,
            Json(ErrorResponse {
                error: self.to_string(),
            }),
        )
            .into_response()
    }
}
