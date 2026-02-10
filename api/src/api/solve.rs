use actix_web::{web, HttpResponse};

use crate::{
    error::AppError,
    mock_data::{get_mock_actions, get_mock_combos},
    models::{SolveRequest, SolveResponse},
};

/// Solve endpoint - returns Nash-equilibrium strategy for a game state
///
/// Currently returns **mock data** — all 46 combos from the example
/// OOP range on board Ah Kd Qc. The real solver will be wired in later
/// without changing the API shape.
#[utoipa::path(
    post,
    path = "/v1/solve",
    request_body = SolveRequest,
    responses(
        (status = 200, description = "Successfully computed strategy", body = SolveResponse),
        (status = 422, description = "Validation error", body = crate::error::ErrorDetail)
    ),
    tag = "Solver"
)]
pub async fn solve(
    req: web::Json<SolveRequest>,
) -> Result<HttpResponse, AppError> {
    // Get mock data
    let actions = get_mock_actions();
    let combos = get_mock_combos();

    // Extract inner SolveRequest
    let req = req.into_inner();

    // Build response matching the request
    let response = SolveResponse {
        player: req.player,
        board: req.board,
        pot: req.starting_pot,
        effective_stack: req.effective_stack,
        num_combos: combos.len(),
        actions,
        combos,
    };

    Ok(HttpResponse::Ok().json(response))
}
