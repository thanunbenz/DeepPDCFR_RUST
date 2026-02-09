use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::request::Player;

/// Action type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ActionTypeResponse {
    Fold,
    Check,
    Call,
    Bet,
    Raise,
    Allin,
}

/// Description of an available action at a decision node
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ActionInfo {
    /// Human-readable action name
    #[schema(example = "Bet 33%")]
    pub name: String,

    /// Semantic action type
    #[schema(example = "bet")]
    #[serde(rename = "type")]
    pub action_type: ActionTypeResponse,

    /// Amount in big blinds (0 for check/fold)
    #[schema(example = 6.6)]
    pub amount_big_blinds: f64,

    /// Amount as percentage of pot (0 for check/fold)
    #[schema(example = 33.0)]
    pub amount_percent: f64,

    /// Average frequency of this action across all combos (0.0–1.0)
    #[schema(example = 0.45)]
    #[serde(default)]
    pub frequency: f64,
}

/// Strategy for a single combo (hand)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HandStrategy {
    /// Hand in card notation. e.g. 'AhKd'
    #[schema(example = "QdQs")]
    pub hand: String,

    /// Internal combo ID (0-1325)
    #[schema(example = 820)]
    pub hand_id: u32,

    /// Action probabilities matching the 'actions' array order. Sums to 1.0.
    #[schema(example = json!([0.05, 0.25, 0.55, 0.15]))]
    pub strategy: Vec<f64>,
}

/// Response body for POST /v1/solve
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SolveResponse {
    /// Acting player at this node
    #[schema(example = "OOP")]
    pub player: Player,

    /// Board cards (space-separated)
    #[schema(example = "Ah Kd Qc")]
    pub board: String,

    /// Current pot size in big blinds (bb)
    #[schema(example = 20)]
    pub pot: u32,

    /// Effective stack in big blinds (bb)
    #[schema(example = 100)]
    pub effective_stack: u32,

    /// Number of combos returned
    #[schema(example = 15)]
    pub num_combos: usize,

    /// Available actions at this node
    pub actions: Vec<ActionInfo>,

    /// Per-combo strategy
    pub combos: Vec<HandStrategy>,
}
