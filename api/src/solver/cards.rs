//! Card representation and combo generation for poker hands
//!
//! This module provides:
//! - Compact card encoding (52 cards as u8)
//! - Rank and Suit enums
//! - String parsing ("Ah", "Kd", etc.)
//! - All 1326 hand combinations
//! - Blocking logic for boards

use std::fmt;
use std::str::FromStr;

/// Card rank (2 through Ace)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Rank {
    /// Convert rank to character
    pub fn to_char(self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }

    /// Parse rank from character
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            '9' => Some(Rank::Nine),
            'T' | 't' => Some(Rank::Ten),
            'J' | 'j' => Some(Rank::Jack),
            'Q' | 'q' => Some(Rank::Queen),
            'K' | 'k' => Some(Rank::King),
            'A' | 'a' => Some(Rank::Ace),
            _ => None,
        }
    }

    /// Get all ranks
    pub fn all() -> [Rank; 13] {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Card suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

impl Suit {
    /// Convert suit to character
    pub fn to_char(self) -> char {
        match self {
            Suit::Clubs => 'c',
            Suit::Diamonds => 'd',
            Suit::Hearts => 'h',
            Suit::Spades => 's',
        }
    }

    /// Parse suit from character
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'c' | 'C' => Some(Suit::Clubs),
            'd' | 'D' => Some(Suit::Diamonds),
            'h' | 'H' => Some(Suit::Hearts),
            's' | 'S' => Some(Suit::Spades),
            _ => None,
        }
    }

    /// Get all suits
    pub fn all() -> [Suit; 4] {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// A playing card (compact u8 encoding)
///
/// Cards are encoded as: rank * 4 + suit
/// This gives values 0-51 for all 52 cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(u8);

impl Card {
    /// Create a new card from rank and suit
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card((rank as u8) * 4 + (suit as u8))
    }

    /// Get the rank of this card
    pub fn rank(self) -> Rank {
        // SAFETY: rank is always 0-12 by construction
        unsafe { std::mem::transmute(self.0 / 4) }
    }

    /// Get the suit of this card
    pub fn suit(self) -> Suit {
        // SAFETY: suit is always 0-3 by construction
        unsafe { std::mem::transmute(self.0 % 4) }
    }

    /// Get the internal card value (0-51)
    pub fn value(self) -> u8 {
        self.0
    }

    /// Create a card from internal value
    pub fn from_value(value: u8) -> Option<Self> {
        if value < 52 {
            Some(Card(value))
        } else {
            None
        }
    }
}

impl FromStr for Card {
    type Err = String;

    /// Parse a card from string like "Ah", "Kd", "2c"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(format!("Invalid card string: '{}' (expected 2 characters)", s));
        }

        let rank = Rank::from_char(chars[0])
            .ok_or_else(|| format!("Invalid rank: '{}'", chars[0]))?;
        let suit = Suit::from_char(chars[1])
            .ok_or_else(|| format!("Invalid suit: '{}'", chars[1]))?;

        Ok(Card::new(rank, suit))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

/// A two-card combination (hole cards)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Combo {
    pub card1: Card,
    pub card2: Card,
    pub id: u16,
}

impl Combo {
    /// Create a new combo with the given ID
    pub fn new(card1: Card, card2: Card, id: u16) -> Self {
        Combo { card1, card2, id }
    }

    /// Check if this combo is blocked by any of the given cards
    pub fn is_blocked_by(&self, cards: &[Card]) -> bool {
        cards.contains(&self.card1) || cards.contains(&self.card2)
    }

    /// Get cards as array
    pub fn cards(&self) -> [Card; 2] {
        [self.card1, self.card2]
    }
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.card1, self.card2)
    }
}

/// Generate all 1326 possible two-card combinations
///
/// Combos are ordered from highest to lowest (AA first, 22 last)
/// Each combo is assigned a unique ID from 0 to 1325
pub fn generate_all_combos() -> Vec<Combo> {
    let mut combos = Vec::with_capacity(1326);
    let mut id = 0u16;

    // Generate all 52 cards (highest rank first for correct ordering)
    let mut all_cards = Vec::with_capacity(52);
    for rank in Rank::all().iter().rev() {
        for suit in Suit::all().iter().rev() {
            all_cards.push(Card::new(*rank, *suit));
        }
    }

    // Generate all combinations (choose 2 from 52)
    for i in 0..52 {
        for j in (i + 1)..52 {
            let card1 = all_cards[i]; // Higher card first
            let card2 = all_cards[j];
            combos.push(Combo::new(card1, card2, id));
            id += 1;
        }
    }

    combos
}

/// Filter combos that are not blocked by the given board cards
pub fn filter_blocked_combos(combos: &[Combo], board: &[Card]) -> Vec<Combo> {
    combos
        .iter()
        .filter(|combo| !combo.is_blocked_by(board))
        .copied()
        .collect()
}

/// Parse board cards from string
///
/// Supports both space-separated ("Ah Kd Qc") and concatenated ("AhKdQc") formats
pub fn parse_board(s: &str) -> Result<Vec<Card>, String> {
    let s = s.trim();

    // Try space-separated first
    if s.contains(' ') {
        s.split_whitespace()
            .map(|card_str| card_str.parse())
            .collect()
    } else {
        // Try concatenated format (every 2 characters)
        if s.len() % 2 != 0 {
            return Err(format!("Invalid board string length: {}", s.len()));
        }

        let mut cards = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        for i in (0..chars.len()).step_by(2) {
            let card_str: String = chars[i..i + 2].iter().collect();
            cards.push(card_str.parse()?);
        }
        Ok(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_conversion() {
        assert_eq!(Rank::Ace.to_char(), 'A');
        assert_eq!(Rank::from_char('A'), Some(Rank::Ace));
        assert_eq!(Rank::from_char('a'), Some(Rank::Ace));
        assert_eq!(Rank::from_char('K'), Some(Rank::King));
        assert_eq!(Rank::from_char('2'), Some(Rank::Two));
        assert_eq!(Rank::from_char('X'), None);
    }

    #[test]
    fn test_suit_conversion() {
        assert_eq!(Suit::Hearts.to_char(), 'h');
        assert_eq!(Suit::from_char('h'), Some(Suit::Hearts));
        assert_eq!(Suit::from_char('H'), Some(Suit::Hearts));
        assert_eq!(Suit::from_char('s'), Some(Suit::Spades));
        assert_eq!(Suit::from_char('X'), None);
    }

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Hearts);
        assert_eq!(card.to_string(), "Ah");
    }

    #[test]
    fn test_card_parsing() {
        let card: Card = "Ah".parse().unwrap();
        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Hearts);

        let card: Card = "2c".parse().unwrap();
        assert_eq!(card.rank(), Rank::Two);
        assert_eq!(card.suit(), Suit::Clubs);

        assert!("X".parse::<Card>().is_err());
        assert!("Ahh".parse::<Card>().is_err());
    }

    #[test]
    fn test_combo_generation() {
        let combos = generate_all_combos();
        assert_eq!(combos.len(), 1326); // C(52, 2) = 1326

        // Check first combo (highest cards)
        let first = combos.first().unwrap();
        assert_eq!(first.card1.rank(), Rank::Ace);
        assert_eq!(first.card2.rank(), Rank::Ace);
        assert_eq!(first.id, 0);

        // Check last combo (lowest cards)
        let last = combos.last().unwrap();
        assert_eq!(last.card1.rank(), Rank::Two);
        assert_eq!(last.card2.rank(), Rank::Two);
        assert_eq!(last.id, 1325);
    }

    #[test]
    fn test_blocking() {
        let combos = generate_all_combos();
        let board = vec!["Ah".parse().unwrap(), "Kd".parse().unwrap()];

        let unblocked = filter_blocked_combos(&combos, &board);

        // Should have fewer combos
        assert!(unblocked.len() < combos.len());

        // No combo should contain Ah or Kd
        for combo in &unblocked {
            assert!(!combo.is_blocked_by(&board));
        }

        // AA combos without Ah: should have 5 (AcAd, AcAs, AdAs, plus 2 with Kx but Kd blocked)
        // Actually: AA without Ah = 5 combos (missing AhAc, AhAd, AhAs)
        let aa_combos: Vec<_> = unblocked
            .iter()
            .filter(|c| c.card1.rank() == Rank::Ace && c.card2.rank() == Rank::Ace)
            .collect();
        assert_eq!(aa_combos.len(), 3); // AcAd, AcAs, AdAs
    }

    #[test]
    fn test_parse_board_space_separated() {
        let board = parse_board("Ah Kd Qc").unwrap();
        assert_eq!(board.len(), 3);
        assert_eq!(board[0].to_string(), "Ah");
        assert_eq!(board[1].to_string(), "Kd");
        assert_eq!(board[2].to_string(), "Qc");
    }

    #[test]
    fn test_parse_board_concatenated() {
        let board = parse_board("AhKdQc").unwrap();
        assert_eq!(board.len(), 3);
        assert_eq!(board[0].to_string(), "Ah");
        assert_eq!(board[1].to_string(), "Kd");
        assert_eq!(board[2].to_string(), "Qc");
    }

    #[test]
    fn test_parse_board_invalid() {
        assert!(parse_board("AhKdQ").is_err()); // Odd length
        assert!(parse_board("Ah Xd").is_err()); // Invalid card
    }
}
