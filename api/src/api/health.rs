use actix_web::HttpResponse;
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
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse::default())
}
