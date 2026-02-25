//! Bet sizing parser for PioSOLVER-style bet size configurations
//!
//! Supports syntax like:
//! - Percentage of pot: "33", "67", "100"
//! - All-in: "a" or "allin"
//! - Multiple sizes: "33, 67, a"

use crate::models::BetSizes;

/// A bet size specification
#[derive(Debug, Clone, PartialEq)]
pub enum BetSize {
    /// Percentage of pot (e.g., 33 for 33% pot)
    Percent(f64),
    /// All-in (effective stack)
    AllIn,
}

impl BetSize {
    /// Calculate the actual bet amount in big blinds
    pub fn calculate(&self, pot: u32, stack: u32) -> u32 {
        match self {
            BetSize::Percent(pct) => {
                let amount = (pot as f64 * pct / 100.0).round() as u32;
                amount.min(stack) // Cap at stack
            }
            BetSize::AllIn => stack,
        }
    }
}

/// Bet size configuration for all situations
#[derive(Debug, Clone)]
pub struct BetSizeConfig {
    /// OOP bet sizes
    pub oop_bet: Vec<BetSize>,
    /// OOP raise sizes
    pub oop_raise: Vec<BetSize>,
    /// IP bet sizes
    pub ip_bet: Vec<BetSize>,
    /// IP raise sizes
    pub ip_raise: Vec<BetSize>,
}

impl BetSizeConfig {
    /// Parse bet sizes from API model
    pub fn from_bet_sizes(bet_sizes: &BetSizes) -> Result<Self, String> {
        Ok(BetSizeConfig {
            oop_bet: parse_bet_size_string(&bet_sizes.oop_bet)?,
            oop_raise: parse_bet_size_string(&bet_sizes.oop_raise)?,
            ip_bet: parse_bet_size_string(&bet_sizes.ip_bet)?,
            ip_raise: parse_bet_size_string(&bet_sizes.ip_raise)?,
        })
    }

    /// Get bet amounts for a given pot and stack
    pub fn get_bet_amounts(&self, oop: bool, pot: u32, stack: u32) -> Vec<u32> {
        let sizes = if oop { &self.oop_bet } else { &self.ip_bet };
        sizes
            .iter()
            .map(|size| size.calculate(pot, stack))
            .filter(|&amount| amount > 0 && amount <= stack)
            .collect()
    }

    /// Get raise amounts for a given pot, amount to call, and stack
    pub fn get_raise_amounts(&self, oop: bool, pot: u32, to_call: u32, stack: u32) -> Vec<u32> {
        let sizes = if oop {
            &self.oop_raise
        } else {
            &self.ip_raise
        };

        let pot_after_call = pot + to_call;

        sizes
            .iter()
            .map(|size| match size {
                BetSize::Percent(pct) => {
                    // Raise is: call + (pot_after_call * percentage)
                    let raise_amount = (pot_after_call as f64 * pct / 100.0).round() as u32;
                    to_call + raise_amount
                }
                BetSize::AllIn => stack,
            })
            .filter(|&amount| amount > to_call && amount <= stack)
            .collect()
    }
}

impl Default for BetSizeConfig {
    fn default() -> Self {
        BetSizeConfig {
            oop_bet: parse_bet_size_string("33, 67, a").unwrap(),
            oop_raise: parse_bet_size_string("50, a").unwrap(),
            ip_bet: parse_bet_size_string("33, 67, a").unwrap(),
            ip_raise: parse_bet_size_string("50, a").unwrap(),
        }
    }
}

/// Parse a bet size string like "33, 67, a"
fn parse_bet_size_string(s: &str) -> Result<Vec<BetSize>, String> {
    let mut sizes = Vec::new();

    for token in s.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        let size = parse_single_bet_size(token)?;
        sizes.push(size);
    }

    if sizes.is_empty() {
        return Err("Bet size string cannot be empty".to_string());
    }

    Ok(sizes)
}

/// Parse a single bet size token
fn parse_single_bet_size(s: &str) -> Result<BetSize, String> {
    let s = s.trim();

    // Check for all-in
    if s.eq_ignore_ascii_case("a") || s.eq_ignore_ascii_case("allin") {
        return Ok(BetSize::AllIn);
    }

    // Parse as percentage
    let value = s
        .parse::<f64>()
        .map_err(|_| format!("Invalid bet size: '{}' (expected number or 'a')", s))?;

    if value <= 0.0 {
        return Err(format!("Bet size must be positive, got {}", value));
    }

    Ok(BetSize::Percent(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_percent() {
        let size = parse_single_bet_size("33").unwrap();
        assert_eq!(size, BetSize::Percent(33.0));
    }

    #[test]
    fn test_parse_allin() {
        assert_eq!(parse_single_bet_size("a").unwrap(), BetSize::AllIn);
        assert_eq!(parse_single_bet_size("allin").unwrap(), BetSize::AllIn);
        assert_eq!(parse_single_bet_size("ALLIN").unwrap(), BetSize::AllIn);
    }

    #[test]
    fn test_parse_multiple() {
        let sizes = parse_bet_size_string("33, 67, a").unwrap();
        assert_eq!(sizes.len(), 3);
        assert_eq!(sizes[0], BetSize::Percent(33.0));
        assert_eq!(sizes[1], BetSize::Percent(67.0));
        assert_eq!(sizes[2], BetSize::AllIn);
    }

    #[test]
    fn test_parse_decimal() {
        let size = parse_single_bet_size("33.5").unwrap();
        assert_eq!(size, BetSize::Percent(33.5));
    }

    #[test]
    fn test_calculate_percent() {
        let size = BetSize::Percent(33.0);
        // 33% of pot=100 = 33bb
        assert_eq!(size.calculate(100, 200), 33);

        // Capped at stack
        assert_eq!(size.calculate(100, 20), 20);
    }

    #[test]
    fn test_calculate_allin() {
        let size = BetSize::AllIn;
        assert_eq!(size.calculate(100, 50), 50);
    }

    #[test]
    fn test_get_bet_amounts() {
        let config = BetSizeConfig::default();
        let amounts = config.get_bet_amounts(true, 20, 100);

        // 33% of 20 = 6.6 ≈ 7bb
        // 67% of 20 = 13.4 ≈ 13bb
        // a = 100bb
        assert_eq!(amounts.len(), 3);
        assert!(amounts.contains(&7) || amounts.contains(&6)); // Rounding
        assert!(amounts.contains(&13) || amounts.contains(&14));
        assert!(amounts.contains(&100));
    }

    #[test]
    fn test_get_raise_amounts() {
        let config = BetSizeConfig::default();
        // Pot=20, to_call=10, stack=100
        // pot_after_call = 20 + 10 = 30
        // 50% raise = call(10) + 50% of pot_after_call(15) = 25
        let amounts = config.get_raise_amounts(true, 20, 10, 100);

        assert!(amounts.len() >= 1);
        // Should include all-in
        assert!(amounts.contains(&100));
    }

    #[test]
    fn test_invalid_bet_size() {
        assert!(parse_single_bet_size("xyz").is_err());
        assert!(parse_single_bet_size("-10").is_err());
        assert!(parse_single_bet_size("0").is_err());
    }

    #[test]
    fn test_empty_string() {
        assert!(parse_bet_size_string("").is_err());
        assert!(parse_bet_size_string("   ").is_err());
    }

    #[test]
    fn test_from_bet_sizes_model() {
        let model = BetSizes {
            oop_bet: "33, 67, a".to_string(),
            oop_raise: "50, a".to_string(),
            ip_bet: "33, 67, a".to_string(),
            ip_raise: "50, a".to_string(),
        };

        let config = BetSizeConfig::from_bet_sizes(&model).unwrap();
        assert_eq!(config.oop_bet.len(), 3);
        assert_eq!(config.oop_raise.len(), 2);
    }
}
