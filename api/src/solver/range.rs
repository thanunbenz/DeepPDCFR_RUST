//! PioSOLVER-style range parsing for poker hands
//!
//! Supports syntax like:
//! - Specific hands: "AA", "AKs", "AKo"
//! - Ranges: "JJ-99", "AQs-ATs"
//! - Frequencies: "QQ:0.5", "AA:0.75"
//! - Plus notation: "22+", "A2s+", "ATo+"
//! - Combinations: "AA,KK,QQ,JJ-99,AQs-ATs"

use std::collections::HashMap;

use super::cards::{Card, Combo, Rank, generate_all_combos};

/// A hand range with combo frequencies
#[derive(Debug, Clone)]
pub struct Range {
    /// Map from combo ID to frequency (0.0-1.0)
    combos: HashMap<u16, f64>,
}

impl Range {
    /// Create an empty range
    pub fn new() -> Self {
        Range {
            combos: HashMap::new(),
        }
    }

    /// Parse a range from PioSOLVER syntax
    ///
    /// Examples:
    /// - "AA" - all pocket aces
    /// - "AKs" - all suited AK combinations
    /// - "AKo" - all offsuit AK combinations
    /// - "QQ:0.5" - pocket queens at 50% frequency
    /// - "JJ-99" - pocket jacks through nines
    /// - "22+" - all pocket pairs
    /// - "AA,KK,QQ" - multiple hands
    pub fn parse(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Ok(Range::new());
        }

        let mut range = Range::new();

        // Split by comma
        for token in s.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }

            // Check for frequency suffix (e.g., "QQ:0.5")
            let (hand_str, frequency) = if let Some(colon_pos) = token.find(':') {
                let hand = &token[..colon_pos];
                let freq_str = &token[colon_pos + 1..];
                let freq = freq_str
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid frequency: '{}'", freq_str))?;
                if !(0.0..=1.0).contains(&freq) {
                    return Err(format!("Frequency must be 0.0-1.0, got {}", freq));
                }
                (hand, freq)
            } else {
                (token, 1.0)
            };

            // Parse the hand pattern
            let combo_ids = parse_hand_pattern(hand_str)?;

            // Add to range
            for combo_id in combo_ids {
                range.combos.insert(combo_id, frequency);
            }
        }

        Ok(range)
    }

    /// Filter combos blocked by the given cards
    pub fn filter_blocked(&self, board: &[Card]) -> Range {
        let all_combos = generate_all_combos();
        let mut filtered = Range::new();

        for (&combo_id, &frequency) in &self.combos {
            let combo = &all_combos[combo_id as usize];
            if !combo.is_blocked_by(board) {
                filtered.combos.insert(combo_id, frequency);
            }
        }

        filtered
    }

    /// Get all combos with their frequencies
    pub fn get_combos(&self) -> Vec<(u16, f64)> {
        self.combos.iter().map(|(&id, &freq)| (id, freq)).collect()
    }

    /// Check if range is empty
    pub fn is_empty(&self) -> bool {
        self.combos.is_empty()
    }

    /// Get number of combos
    pub fn len(&self) -> usize {
        self.combos.len()
    }

    /// Get frequency of a specific combo
    pub fn get_frequency(&self, combo_id: u16) -> f64 {
        self.combos.get(&combo_id).copied().unwrap_or(0.0)
    }
}

impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a single hand pattern into combo IDs
fn parse_hand_pattern(s: &str) -> Result<Vec<u16>, String> {
    let all_combos = generate_all_combos();

    // Check for plus notation (e.g., "22+", "A2s+", "ATo+")
    if s.ends_with('+') {
        return parse_plus_notation(&s[..s.len() - 1]);
    }

    // Check for range (e.g., "JJ-99", "AQs-ATs")
    if s.contains('-') {
        return parse_range_notation(s);
    }

    // Single hand (e.g., "AA", "AKs", "AKo")
    parse_single_hand(s, &all_combos)
}

/// Parse single hand like "AA", "AKs", "AKo"
fn parse_single_hand(s: &str, all_combos: &[Combo]) -> Result<Vec<u16>, String> {
    let chars: Vec<char> = s.chars().collect();

    if chars.len() < 2 {
        return Err(format!("Invalid hand: '{}'", s));
    }

    let rank1 = Rank::from_char(chars[0])
        .ok_or_else(|| format!("Invalid rank: '{}'", chars[0]))?;
    let rank2 = Rank::from_char(chars[1])
        .ok_or_else(|| format!("Invalid rank: '{}'", chars[1]))?;

    // Check for suited/offsuit modifier
    let suited_filter = if chars.len() == 3 {
        match chars[2] {
            's' | 'S' => Some(true),  // Suited only
            'o' | 'O' => Some(false), // Offsuit only
            _ => return Err(format!("Invalid modifier: '{}' (expected 's' or 'o')", chars[2])),
        }
    } else if chars.len() == 2 {
        None // Both suited and offsuit (for pairs)
    } else {
        return Err(format!("Invalid hand format: '{}'", s));
    };

    // Find matching combos
    let mut combo_ids = Vec::new();
    for combo in all_combos {
        let c1_rank = combo.card1.rank();
        let c2_rank = combo.card2.rank();

        // Match ranks (higher rank first in combo)
        let ranks_match = if rank1 >= rank2 {
            c1_rank == rank1 && c2_rank == rank2
        } else {
            c1_rank == rank2 && c2_rank == rank1
        };

        if !ranks_match {
            continue;
        }

        // Check suited/offsuit
        let is_suited = combo.card1.suit() == combo.card2.suit();
        match suited_filter {
            Some(true) if !is_suited => continue,  // Want suited, got offsuit
            Some(false) if is_suited => continue,  // Want offsuit, got suited
            _ => {}
        }

        combo_ids.push(combo.id);
    }

    if combo_ids.is_empty() {
        Err(format!("No combos found for hand: '{}'", s))
    } else {
        Ok(combo_ids)
    }
}

/// Parse plus notation like "22+", "A2s+", "ATo+"
fn parse_plus_notation(s: &str) -> Result<Vec<u16>, String> {
    let all_combos = generate_all_combos();
    let chars: Vec<char> = s.chars().collect();

    if chars.len() < 2 {
        return Err(format!("Invalid plus notation: '{}+'", s));
    }

    let rank1 = Rank::from_char(chars[0])
        .ok_or_else(|| format!("Invalid rank: '{}'", chars[0]))?;
    let rank2 = Rank::from_char(chars[1])
        .ok_or_else(|| format!("Invalid rank: '{}'", chars[1]))?;

    // Determine if suited/offsuit
    let suited_filter = if chars.len() == 3 {
        match chars[2] {
            's' | 'S' => Some(true),
            'o' | 'O' => Some(false),
            _ => return Err(format!("Invalid modifier: '{}'", chars[2])),
        }
    } else {
        None
    };

    let mut combo_ids = Vec::new();

    // For pairs (e.g., "22+"), include all pairs >= rank
    if rank1 == rank2 {
        for rank in (rank1 as u8)..=12 {
            let r = unsafe { std::mem::transmute::<u8, Rank>(rank) };
            let hand_str = format!("{}{}", r.to_char(), r.to_char());
            combo_ids.extend(parse_single_hand(&hand_str, &all_combos)?);
        }
    } else {
        // For non-pairs (e.g., "A2s+"), include all hands with first rank and >= second rank
        for rank in (rank2 as u8)..rank1 as u8 {
            let r = unsafe { std::mem::transmute::<u8, Rank>(rank) };
            let modifier = match suited_filter {
                Some(true) => "s",
                Some(false) => "o",
                None => "",
            };
            let hand_str = format!("{}{}{}", rank1.to_char(), r.to_char(), modifier);
            combo_ids.extend(parse_single_hand(&hand_str, &all_combos)?);
        }
    }

    Ok(combo_ids)
}

/// Parse range notation like "JJ-99", "AQs-ATs"
fn parse_range_notation(s: &str) -> Result<Vec<u16>, String> {
    let all_combos = generate_all_combos();
    let parts: Vec<&str> = s.split('-').collect();

    if parts.len() != 2 {
        return Err(format!("Invalid range format: '{}'", s));
    }

    let start_chars: Vec<char> = parts[0].chars().collect();
    let end_chars: Vec<char> = parts[1].chars().collect();

    if start_chars.len() < 2 || end_chars.len() < 2 {
        return Err(format!("Invalid range: '{}'", s));
    }

    let start_rank1 = Rank::from_char(start_chars[0])
        .ok_or_else(|| format!("Invalid rank: '{}'", start_chars[0]))?;
    let start_rank2 = Rank::from_char(start_chars[1])
        .ok_or_else(|| format!("Invalid rank: '{}'", start_chars[1]))?;

    let end_rank1 = Rank::from_char(end_chars[0])
        .ok_or_else(|| format!("Invalid rank: '{}'", end_chars[0]))?;
    let end_rank2 = Rank::from_char(end_chars[1])
        .ok_or_else(|| format!("Invalid rank: '{}'", end_chars[1]))?;

    // Determine suited/offsuit modifier
    let suited_filter = if start_chars.len() == 3 {
        match start_chars[2] {
            's' | 'S' => Some(true),
            'o' | 'O' => Some(false),
            _ => return Err(format!("Invalid modifier: '{}'", start_chars[2])),
        }
    } else {
        None
    };

    let mut combo_ids = Vec::new();

    // For pairs (e.g., "JJ-99")
    if start_rank1 == start_rank2 && end_rank1 == end_rank2 {
        let start = std::cmp::min(start_rank1 as u8, end_rank1 as u8);
        let end = std::cmp::max(start_rank1 as u8, end_rank1 as u8);

        for rank in start..=end {
            let r = unsafe { std::mem::transmute::<u8, Rank>(rank) };
            let hand_str = format!("{}{}", r.to_char(), r.to_char());
            combo_ids.extend(parse_single_hand(&hand_str, &all_combos)?);
        }
    } else {
        // For non-pairs (e.g., "AQs-ATs")
        // Assume same first rank, range on second rank
        if start_rank1 != end_rank1 {
            return Err(format!("Range must have same first rank: '{}'", s));
        }

        let start = std::cmp::min(start_rank2 as u8, end_rank2 as u8);
        let end = std::cmp::max(start_rank2 as u8, end_rank2 as u8);

        for rank in start..=end {
            let r = unsafe { std::mem::transmute::<u8, Rank>(rank) };
            let modifier = match suited_filter {
                Some(true) => "s",
                Some(false) => "o",
                None => "",
            };
            let hand_str = format!("{}{}{}", start_rank1.to_char(), r.to_char(), modifier);
            combo_ids.extend(parse_single_hand(&hand_str, &all_combos)?);
        }
    }

    Ok(combo_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_pair() {
        let range = Range::parse("AA").unwrap();
        assert_eq!(range.len(), 6); // 6 combos of AA
        assert_eq!(range.get_frequency(0), 1.0);
    }

    #[test]
    fn test_parse_suited() {
        let range = Range::parse("AKs").unwrap();
        assert_eq!(range.len(), 4); // 4 suited combos
    }

    #[test]
    fn test_parse_offsuit() {
        let range = Range::parse("AKo").unwrap();
        assert_eq!(range.len(), 12); // 12 offsuit combos
    }

    #[test]
    fn test_parse_with_frequency() {
        let range = Range::parse("QQ:0.5").unwrap();
        assert_eq!(range.len(), 6);
        for (_, freq) in range.get_combos() {
            assert_eq!(freq, 0.5);
        }
    }

    #[test]
    fn test_parse_multiple() {
        let range = Range::parse("AA,KK,QQ").unwrap();
        assert_eq!(range.len(), 18); // 6 + 6 + 6
    }

    #[test]
    fn test_parse_pair_range() {
        let range = Range::parse("JJ-99").unwrap();
        assert_eq!(range.len(), 18); // 6 * 3 pairs
    }

    #[test]
    fn test_parse_plus_pairs() {
        let range = Range::parse("QQ+").unwrap();
        assert_eq!(range.len(), 18); // QQ, KK, AA = 6*3
    }

    #[test]
    fn test_filter_blocked() {
        let range = Range::parse("AA").unwrap();
        let board = vec!["Ah".parse().unwrap()];

        let filtered = range.filter_blocked(&board);
        assert_eq!(filtered.len(), 3); // Only 3 AA combos without Ah
    }

    #[test]
    fn test_complex_range() {
        let range = Range::parse("AA,AKs,AKo,KK,QQ:0.5,JJ-99,AQs-ATs,KQs").unwrap();
        assert!(!range.is_empty());
        assert!(range.len() > 30); // Should have many combos
    }

    #[test]
    fn test_empty_range() {
        let range = Range::parse("").unwrap();
        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn test_invalid_frequency() {
        assert!(Range::parse("AA:1.5").is_err()); // > 1.0
        assert!(Range::parse("AA:-0.1").is_err()); // < 0.0
    }

    #[test]
    fn test_invalid_hand() {
        assert!(Range::parse("XX").is_err());
        assert!(Range::parse("A").is_err());
        assert!(Range::parse("AKx").is_err());
    }
}
