//! Hand evaluation for poker hands

use super::cards::Card;

/// Hand strength value (lower is better)
pub type HandStrength = u16;

/// Hand evaluator
pub struct HandEvaluator;

impl HandEvaluator {
    /// Create a new hand evaluator
    pub fn new() -> Self {
        HandEvaluator
    }

    /// Evaluate a 5-card hand
    pub fn evaluate_5cards(&self, _cards: [Card; 5]) -> HandStrength {
        // TODO: Implement hand evaluation
        0
    }

    /// Evaluate a 7-card hand (5 cards from board + 2 hole cards)
    pub fn evaluate_7cards(&self, _cards: [Card; 7]) -> HandStrength {
        // TODO: Implement 7-card evaluation
        0
    }
}

impl Default for HandEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
