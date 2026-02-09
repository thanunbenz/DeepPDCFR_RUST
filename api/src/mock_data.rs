use crate::models::{ActionInfo, HandStrategy};
use crate::models::response::ActionTypeResponse;

/// Get mock actions for the scenario
pub fn get_mock_actions() -> Vec<ActionInfo> {
    vec![
        ActionInfo {
            name: "Check".to_string(),
            action_type: ActionTypeResponse::Check,
            amount_big_blinds: 0.0,
            amount_percent: 0.0,
            frequency: 0.0,
        },
        ActionInfo {
            name: "Bet 33%".to_string(),
            action_type: ActionTypeResponse::Bet,
            amount_big_blinds: 6.6,
            amount_percent: 33.0,
            frequency: 0.0,
        },
        ActionInfo {
            name: "Bet 67%".to_string(),
            action_type: ActionTypeResponse::Bet,
            amount_big_blinds: 13.4,
            amount_percent: 67.0,
            frequency: 0.0,
        },
        ActionInfo {
            name: "All-in".to_string(),
            action_type: ActionTypeResponse::Allin,
            amount_big_blinds: 100.0,
            amount_percent: 500.0,
            frequency: 0.0,
        },
    ]
}

/// Get mock hand combinations with strategies
/// Mock scenario: OOP opening on flop Ah Kd Qc
/// pot = 20bb, effective_stack = 100bb
/// oop_range = AA,AKs,AKo,KK,QQ:0.5,JJ-99,AQs-ATs,KQs
/// Every combo below is in the OOP range and not blocked by the board.
/// Strategies are [Check, Bet 33%, Bet 67%, All-in] — all sum to 1.0.
pub fn get_mock_combos() -> Vec<HandStrategy> {
    vec![
        // --- AA (3 combos) — trips aces, mix trap + value bet ---
        HandStrategy { hand: "AcAd".to_string(), hand_id: 1320, strategy: vec![0.15, 0.20, 0.50, 0.15] },
        HandStrategy { hand: "AcAs".to_string(), hand_id: 1322, strategy: vec![0.15, 0.20, 0.50, 0.15] },
        HandStrategy { hand: "AdAs".to_string(), hand_id: 1324, strategy: vec![0.15, 0.20, 0.50, 0.15] },

        // --- AKs (2 combos) — two pair aces and kings ---
        HandStrategy { hand: "AcKc".to_string(), hand_id: 1301, strategy: vec![0.20, 0.45, 0.30, 0.05] },
        HandStrategy { hand: "AsKs".to_string(), hand_id: 1319, strategy: vec![0.20, 0.45, 0.30, 0.05] },

        // --- AKo (7 combos) — two pair aces and kings ---
        HandStrategy { hand: "AcKh".to_string(), hand_id: 1312, strategy: vec![0.25, 0.40, 0.30, 0.05] },
        HandStrategy { hand: "AcKs".to_string(), hand_id: 1316, strategy: vec![0.25, 0.40, 0.30, 0.05] },
        HandStrategy { hand: "AdKc".to_string(), hand_id: 1302, strategy: vec![0.25, 0.40, 0.30, 0.05] },
        HandStrategy { hand: "AdKh".to_string(), hand_id: 1313, strategy: vec![0.25, 0.40, 0.30, 0.05] },
        HandStrategy { hand: "AdKs".to_string(), hand_id: 1317, strategy: vec![0.25, 0.40, 0.30, 0.05] },
        HandStrategy { hand: "AsKc".to_string(), hand_id: 1304, strategy: vec![0.25, 0.40, 0.30, 0.05] },
        HandStrategy { hand: "AsKh".to_string(), hand_id: 1315, strategy: vec![0.25, 0.40, 0.30, 0.05] },

        // --- KK (3 combos) — set of kings ---
        HandStrategy { hand: "KcKh".to_string(), hand_id: 1299, strategy: vec![0.20, 0.25, 0.40, 0.15] },
        HandStrategy { hand: "KcKs".to_string(), hand_id: 1300, strategy: vec![0.20, 0.25, 0.40, 0.15] },
        HandStrategy { hand: "KhKs".to_string(), hand_id: 1311, strategy: vec![0.20, 0.25, 0.40, 0.15] },

        // --- QQ:0.5 (3 combos) — set of queens ---
        HandStrategy { hand: "QdQh".to_string(), hand_id: 1271, strategy: vec![0.10, 0.20, 0.50, 0.20] },
        HandStrategy { hand: "QdQs".to_string(), hand_id: 1272, strategy: vec![0.10, 0.20, 0.50, 0.20] },
        HandStrategy { hand: "QhQs".to_string(), hand_id: 1281, strategy: vec![0.10, 0.20, 0.50, 0.20] },

        // --- AQs (2 combos) — two pair aces and queens ---
        HandStrategy { hand: "AdQd".to_string(), hand_id: 1278, strategy: vec![0.25, 0.40, 0.25, 0.10] },
        HandStrategy { hand: "AsQs".to_string(), hand_id: 1297, strategy: vec![0.25, 0.40, 0.25, 0.10] },

        // --- AJs (3 combos) — top pair + jack kicker ---
        HandStrategy { hand: "AcJc".to_string(), hand_id: 1217, strategy: vec![0.45, 0.35, 0.15, 0.05] },
        HandStrategy { hand: "AdJd".to_string(), hand_id: 1232, strategy: vec![0.45, 0.35, 0.15, 0.05] },
        HandStrategy { hand: "AsJs".to_string(), hand_id: 1259, strategy: vec![0.45, 0.35, 0.15, 0.05] },

        // --- ATs (3 combos) — top pair + ten kicker ---
        HandStrategy { hand: "AcTc".to_string(), hand_id: 1151, strategy: vec![0.55, 0.30, 0.12, 0.03] },
        HandStrategy { hand: "AdTd".to_string(), hand_id: 1170, strategy: vec![0.55, 0.30, 0.12, 0.03] },
        HandStrategy { hand: "AsTs".to_string(), hand_id: 1205, strategy: vec![0.55, 0.30, 0.12, 0.03] },

        // --- KQs (2 combos) — two pair kings and queens ---
        HandStrategy { hand: "KhQh".to_string(), hand_id: 1284, strategy: vec![0.30, 0.35, 0.30, 0.05] },
        HandStrategy { hand: "KsQs".to_string(), hand_id: 1293, strategy: vec![0.30, 0.35, 0.30, 0.05] },

        // --- JJ (6 combos) — underpair ---
        HandStrategy { hand: "JcJd".to_string(), hand_id: 1206, strategy: vec![0.65, 0.22, 0.10, 0.03] },
        HandStrategy { hand: "JcJh".to_string(), hand_id: 1207, strategy: vec![0.65, 0.22, 0.10, 0.03] },
        HandStrategy { hand: "JcJs".to_string(), hand_id: 1208, strategy: vec![0.65, 0.22, 0.10, 0.03] },
        HandStrategy { hand: "JdJh".to_string(), hand_id: 1221, strategy: vec![0.65, 0.22, 0.10, 0.03] },
        HandStrategy { hand: "JdJs".to_string(), hand_id: 1222, strategy: vec![0.65, 0.22, 0.10, 0.03] },
        HandStrategy { hand: "JhJs".to_string(), hand_id: 1235, strategy: vec![0.65, 0.22, 0.10, 0.03] },

        // --- TT (6 combos) — underpair ---
        HandStrategy { hand: "TcTd".to_string(), hand_id: 1136, strategy: vec![0.70, 0.18, 0.09, 0.03] },
        HandStrategy { hand: "TcTh".to_string(), hand_id: 1137, strategy: vec![0.70, 0.18, 0.09, 0.03] },
        HandStrategy { hand: "TcTs".to_string(), hand_id: 1138, strategy: vec![0.70, 0.18, 0.09, 0.03] },
        HandStrategy { hand: "TdTh".to_string(), hand_id: 1155, strategy: vec![0.70, 0.18, 0.09, 0.03] },
        HandStrategy { hand: "TdTs".to_string(), hand_id: 1156, strategy: vec![0.70, 0.18, 0.09, 0.03] },
        HandStrategy { hand: "ThTs".to_string(), hand_id: 1173, strategy: vec![0.70, 0.18, 0.09, 0.03] },

        // --- 99 (6 combos) — underpair ---
        HandStrategy { hand: "9c9d".to_string(), hand_id: 1050, strategy: vec![0.75, 0.15, 0.08, 0.02] },
        HandStrategy { hand: "9c9h".to_string(), hand_id: 1051, strategy: vec![0.75, 0.15, 0.08, 0.02] },
        HandStrategy { hand: "9c9s".to_string(), hand_id: 1052, strategy: vec![0.75, 0.15, 0.08, 0.02] },
        HandStrategy { hand: "9d9h".to_string(), hand_id: 1073, strategy: vec![0.75, 0.15, 0.08, 0.02] },
        HandStrategy { hand: "9d9s".to_string(), hand_id: 1074, strategy: vec![0.75, 0.15, 0.08, 0.02] },
        HandStrategy { hand: "9h9s".to_string(), hand_id: 1095, strategy: vec![0.75, 0.15, 0.08, 0.02] },
    ]
}
