pub mod api;
pub mod config;
pub mod error;
pub mod mock_data;
pub mod models;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{health, solve},
    config::Config,
    models::{
        health::HealthResponse,
        request::{BetSizes, HistoryAction, Player, SolveRequest},
        response::{ActionInfo, HandStrategy, SolveResponse},
    },
};

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "DeepPDCFR Solver API",
        description = "REST API for querying Nash-equilibrium strategies in No-Limit Hold'em. Uses PioSOLVER syntax for bet sizes and hand ranges.",
        version = "0.1.0"
    ),
    paths(
        api::health::health,
        api::solve::solve,
    ),
    components(
        schemas(
            HealthResponse,
            SolveRequest,
            SolveResponse,
            BetSizes,
            HistoryAction,
            Player,
            ActionInfo,
            HandStrategy,
            crate::error::ErrorDetail,
        )
    ),
    tags(
        (name = "System", description = "System endpoints"),
        (name = "Solver", description = "Poker solver endpoints")
    )
)]
struct ApiDoc;

/// Create the application router with all routes and middleware
pub fn create_app() -> Router {
    // Configure CORS to allow all origins (matching Python server)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create API routes
    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/v1/solve", post(solve));

    // Merge routes with Swagger UI
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api_routes)
        .layer(cors)
}

/// Get server configuration
pub fn get_config() -> Config {
    Config::default()
}
