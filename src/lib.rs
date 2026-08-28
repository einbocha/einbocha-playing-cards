use std::{str::FromStr, fmt::{Display, Debug, Formatter}, hash::Hash, num::NonZeroU8};

/// Highly performant playing card representation.
/// Requires just one byte in memory.
/// If an outer datatype must handle optional cards wrap it with an Option<>.
/// The option doesn't add any additional bytes as the playing card uses a NonZeroU8 internally.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayingCard(NonZeroU8);

impl PlayingCard {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let packed: u8 = ((suit as u8) << 4) | (rank as u8);
        Self(
            NonZeroU8::new(packed).unwrap()
        )
    }

    pub fn rank(&self) -> Rank {
        let r: u8 = self.0.get() & 0x0F;
        Rank::from_u8(r)
    }

    pub fn suit(&self) -> Suit {
        let s: u8 = (self.0.get() >> 4) & 0x07;
        Suit::from_u8(s)
    }
}

impl FromStr for PlayingCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.to_lowercase();
        let a: Vec<&str> = s.split_whitespace().collect();
        if a.len() != 2 {
            Err("Invalid playing card.")
        } else {
            Ok(
                PlayingCard::new(
                    Rank::from_str(a[0])?,
                    Suit::from_str(a[1])?,
                )
            )
        }
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.rank(), self.suit())
    }
}

impl Debug for PlayingCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialOrd, PartialEq, Clone, Copy, Ord, Eq)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Rank {
    fn from_u8(u: u8) -> Self {
        match u {
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            14 => Self::Ace,
            _ => unreachable!("Impossible rank bit string."),
        }
    }
}

impl FromStr for Rank {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "10" => Ok(Self::Ten),
            "jack" => Ok(Self::Jack),
            "queen" => Ok(Self::Queen),
            "king" => Ok(Self::King),
            "ace" => Ok(Self::Ace),
            _ => Err("Invalid rank."),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six =>  "6",
            Self::Seven =>  "7",
            Self::Eight =>  "8",
            Self::Nine =>  "9",
            Self::Ten =>  "10",
            Self::Jack =>  "jack",
            Self::Queen => "queen",
            Self::King => "king",
            Self::Ace => "ace",
        };

        (&mut*f).write_str(s)
    }
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Suit {
    Heart = 1,
    Diamond = 2,
    Club = 3,
    Spade = 4,
}

impl Suit {
    fn from_u8(u: u8) -> Self {
        match u {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Club,
            4 => Self::Spade,
            _ => unreachable!("Impossible suit bit string.")
        }
    }
}

impl FromStr for Suit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "heart" => Ok(Self::Heart),
            "diamond" => Ok(Self::Diamond),
            "club" => Ok(Self::Club),
            "spade" => Ok(Self::Spade),
            _ => Err("Invalid suit.")
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = match self {
            Self::Heart => "heart",
            Self::Diamond => "diamond",
            Self::Club => "club",
            Self::Spade => "spade",
        };

        (&mut*f).write_str(s)
    }
}

impl Debug for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub const RANKS: [Rank; 13] = [
    Rank::Two, Rank::Three, Rank::Four,
    Rank::Five, Rank::Six, Rank::Seven,
    Rank::Eight, Rank::Nine, Rank::Ten,
    Rank::Jack, Rank::Queen, Rank::King, Rank::Ace
];
pub const SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];

const fn cartesian_deck<const M: usize, const N: usize, const K: usize>(
    suits: [Suit; M], ranks: [Rank; N]
) -> [PlayingCard; K] {
    let mut result: [PlayingCard; K] = [PlayingCard::new(ranks[0], suits[0]); K];

    let mut idx: usize = 0;
    let mut suit_idx: usize = 0;

    while suit_idx < M {
        let mut rank_idx: usize = 0;
        while rank_idx < N {
            result[idx] = PlayingCard::new(ranks[rank_idx], suits[suit_idx]);
            idx += 1;
            rank_idx += 1;
        }
        suit_idx += 1;
    }

    result
}

pub const DECK_52: [PlayingCard; RANKS.len() * SUITS.len()] = cartesian_deck(SUITS, RANKS);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_deck_small() {
        let suits: [Suit; 2] = [Suit::Diamond, Suit::Club];
        let ranks: [Rank; 3] = [Rank::Two, Rank::Ace, Rank::Ten];
        let result: [PlayingCard; 6] = cartesian_deck(suits, ranks);
        assert_eq!(result, [
            PlayingCard::new(Rank::Two, Suit::Diamond),
            PlayingCard::new(Rank::Ace, Suit::Diamond),
            PlayingCard::new(Rank::Ten, Suit::Diamond),
            PlayingCard::new(Rank::Two, Suit::Club),
            PlayingCard::new(Rank::Ace, Suit::Club),
            PlayingCard::new(Rank::Ten, Suit::Club),
        ]);
    }
}
