//! Game state representation

use super::{BetSizeConfig, Card, Range};
use crate::models::Player;

/// Poker street
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Street {
    Flop = 0,
    Turn = 1,
    River = 2,
}

/// Action in the game
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(u32),   // Amount in bb
    Raise(u32), // Total amount to call
    AllIn(u32),
    Deal(Card), // Progress to next street
}

/// Game state (immutable)
#[derive(Debug, Clone)]
pub struct GameState {
    pub street: Street,
    pub board: Vec<Card>,
    pub pot: u32,
    pub stacks: [u32; 2], // [OOP, IP]
    pub to_act: Player,
    pub oop_range: Range,
    pub ip_range: Range,
    pub history: Vec<Action>,
    pub bet_config: BetSizeConfig,
}

impl GameState {
    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        // TODO: Implement terminal state check
        false
    }

    /// Get available actions
    pub fn get_available_actions(&self) -> Vec<Action> {
        // TODO: Implement action generation
        vec![]
    }

    /// Apply an action to get a new state
    pub fn apply_action(&self, _action: Action) -> GameState {
        // TODO: Implement state transition
        self.clone()
    }
}
