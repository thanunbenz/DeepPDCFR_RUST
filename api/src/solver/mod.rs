/// CFR Solver implementation for No-Limit Hold'em poker
///
/// This module contains the core poker solver using Counterfactual Regret Minimization (CFR)
/// to compute Nash equilibrium strategies.

pub mod cards;
pub mod range;
pub mod bet_sizing;
pub mod game_state;
pub mod hand_eval;
pub mod game_tree;
pub mod cfr;
pub mod utils;

// Re-export commonly used types
pub use cards::{Card, Rank, Suit};
pub use range::Range;
pub use bet_sizing::{BetSize, BetSizeConfig};
pub use game_state::{GameState, Street, Action};
pub use hand_eval::{HandEvaluator, HandStrength};
pub use game_tree::{GameTree, GameNode};
pub use cfr::CFRSolver;

use crate::{
    error::AppError,
    models::{SolveRequest, SolveResponse},
};

/// Solver configuration
#[derive(Debug, Clone)]
pub struct SolverConfig {
    /// Number of CFR iterations to run
    pub iterations: usize,
    /// Maximum tree depth (action limit)
    pub max_depth: usize,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            max_depth: 20,
        }
    }
}

/// Main solver orchestrator
pub struct Solver {
    evaluator: HandEvaluator,
    config: SolverConfig,
}

impl Solver {
    /// Create a new solver with the given configuration
    pub fn new(config: SolverConfig) -> Self {
        Self {
            evaluator: HandEvaluator::new(),
            config,
        }
    }

    /// Solve a poker scenario and return the equilibrium strategy
    pub fn solve(&self, _request: &SolveRequest) -> Result<SolveResponse, AppError> {
        // TODO: Implement full solving pipeline
        // 1. Parse request inputs
        // 2. Build initial game state
        // 3. Build game tree
        // 4. Run CFR iterations
        // 5. Extract and format strategies

        Err(AppError::Internal("Solver not yet implemented".to_string()))
    }
}
