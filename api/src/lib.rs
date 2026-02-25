pub mod api;
pub mod config;
pub mod error;
pub mod mock_data;
pub mod models;
pub mod solver;

use actix_web::web;
use actix_cors::Cors;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{health, solve},
    config::Config,
    error::ErrorDetail,
    models::{
        health::HealthResponse,
        request::{ActionType, BetSizes, HistoryAction, Player, SolveRequest},
        response::{ActionInfo, ActionTypeResponse, HandStrategy, SolveResponse},
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
            ActionType,
            ActionInfo,
            ActionTypeResponse,
            HandStrategy,
            ErrorDetail,
        )
    ),
    tags(
        (name = "System", description = "System endpoints"),
        (name = "Solver", description = "Poker solver endpoints")
    )
)]
struct ApiDoc;

/// Configure application routes and services
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    use actix_web::HttpResponse;

    cfg.route("/health", web::get().to(health))
        .route("/v1/solve", web::post().to(solve))
        // Redirect /docs to /docs/
        .route("/docs", web::get().to(|| async {
            HttpResponse::PermanentRedirect()
                .append_header(("Location", "/docs/"))
                .finish()
        }));
}

/// Create Swagger UI service
pub fn create_swagger() -> SwaggerUi {
    SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

/// Create CORS middleware configuration
pub fn create_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
}

/// Get server configuration
pub fn get_config() -> Config {
    Config::default()
}
