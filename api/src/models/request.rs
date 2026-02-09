use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Player position type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum Player {
    /// Out of position
    OOP,
    /// In position
    IP,
}

/// Bet sizing configuration in PioSOLVER syntax
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BetSizes {
    /// OOP bet sizes (Pio syntax). e.g. "33, 67, a"
    #[schema(example = "33, 67, a")]
    #[serde(default = "default_oop_bet")]
    pub oop_bet: String,

    /// OOP raise sizes (Pio syntax). e.g. "50, a"
    #[schema(example = "50, a")]
    #[serde(default = "default_oop_raise")]
    pub oop_raise: String,

    /// IP bet sizes (Pio syntax). e.g. "33, 67, a"
    #[schema(example = "33, 67, a")]
    #[serde(default = "default_ip_bet")]
    pub ip_bet: String,

    /// IP raise sizes (Pio syntax). e.g. "50, a"
    #[schema(example = "50, a")]
    #[serde(default = "default_ip_raise")]
    pub ip_raise: String,
}

fn default_oop_bet() -> String {
    "33, 67, a".to_string()
}
fn default_oop_raise() -> String {
    "50, a".to_string()
}
fn default_ip_bet() -> String {
    "33, 67, a".to_string()
}
fn default_ip_raise() -> String {
    "50, a".to_string()
}

impl Default for BetSizes {
    fn default() -> Self {
        Self {
            oop_bet: default_oop_bet(),
            oop_raise: default_oop_raise(),
            ip_bet: default_ip_bet(),
            ip_raise: default_ip_raise(),
        }
    }
}

/// Action type in betting history
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Check,
    Call,
    Fold,
    Bet,
    Raise,
    Allin,
    Deal,
}

/// A single action in the betting history
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HistoryAction {
    /// 1-based sequence number of this action
    #[schema(example = 1, minimum = 1)]
    pub order: u32,

    /// Which position made this action
    #[schema(example = "OOP")]
    pub position: Player,

    /// Action type
    #[schema(example = "check")]
    pub action: ActionType,

    /// Bet/raise size as percentage of pot (required for bet/raise, omit otherwise)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// Card dealt (required for deal action only). e.g. '9h'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
}

/// Request body for the POST /v1/solve endpoint
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SolveRequest {
    /// Which player's strategy to return
    #[schema(example = "OOP")]
    pub player: Player,

    /// Board cards. Space-separated or concatenated. e.g. "Ah Kd Qc" or "AhKdQc"
    #[schema(example = "Ah Kd Qc")]
    pub board: String,

    /// Effective stack size in big blinds (bb)
    #[schema(example = 100, minimum = 1)]
    pub effective_stack: u32,

    /// Pot size at the start of the current street in big blinds (bb)
    #[schema(example = 20, minimum = 1)]
    pub starting_pot: u32,

    /// Bet sizing configuration (Pio syntax). Defaults apply if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bet_sizes: Option<BetSizes>,

    /// Betting actions to replay to reach the target node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betting_history: Option<Vec<HistoryAction>>,

    /// OOP range in Pio syntax. e.g. 'AA,AKs,KK,QQ:0.5'. Null = all combos (uniform).
    #[schema(example = "AA,AKs,AKo,KK,QQ:0.5,JJ-99,AQs-ATs,KQs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oop_range: Option<String>,

    /// IP range in Pio syntax. Null = all combos (uniform).
    #[schema(example = "22+,A2s+,K9s+,Q9s+,J9s+,T8s+,97s+,87s,76s,65s,ATo+,KJo+")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
}
