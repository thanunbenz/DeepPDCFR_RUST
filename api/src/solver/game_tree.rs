//! Game tree builder

use super::game_state::{Action, GameState};

/// A node in the game tree
#[derive(Debug, Clone)]
pub struct GameNode {
    pub state: GameState,
    pub children: Vec<(Action, Box<GameNode>)>,
    pub is_terminal: bool,
}

/// Game tree
pub struct GameTree {
    pub root: GameNode,
    pub node_count: usize,
}

impl GameTree {
    /// Build a game tree from initial state
    pub fn build(_initial_state: GameState, _max_depth: usize) -> Self {
        // TODO: Implement tree building
        unimplemented!("Game tree building not yet implemented")
    }
}
