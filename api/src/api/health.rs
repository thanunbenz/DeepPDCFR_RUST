use axum::{response::Json, http::StatusCode};
use crate::models::HealthResponse;

/// Health check endpoint
/// Returns service status, model availability, and version
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    ),
    tag = "System"
)]
pub async fn health() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse::default()))
}
