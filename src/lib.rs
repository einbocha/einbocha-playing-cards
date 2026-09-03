use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    num::NonZeroU8,
    str::FromStr,
};

/// Thrown if no Rank can be derived
#[derive(PartialEq, Debug)]
pub struct RankError;

/// Thrown if no Suit can be derived
#[derive(PartialEq, Debug)]
pub struct SuitError;

/// Thrown if no PlayingCard can be derived
#[derive(PartialEq, Debug)]
pub enum PlayingCardError {
    InvalidRank,
    InvalidSuit,
    InvalidStringFormat,
    InvalidValue,
}

impl From<RankError> for PlayingCardError {
    fn from(_: RankError) -> Self {
        Self::InvalidRank
    }
}

impl From<SuitError> for PlayingCardError {
    fn from(_: SuitError) -> Self {
        Self::InvalidSuit
    }
}

/// Highly performant playing card representation.
/// Stored as a single byte in memory.
/// Wrapping a card in an Option<> doesn't introduce any overhead
/// as the card can be encoded internally using a NonZeroU8.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayingCard(NonZeroU8);

impl PlayingCard {
    /// Returns guaranteed a valid PlayingCard
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let packed: u8 = (suit as u8) * 13 + rank as u8;
        Self(NonZeroU8::new(packed).unwrap())
    }

    // u8: {2, ..., 53} -> PlayingCard
    pub fn from_val(val: u8) -> Result<Self, PlayingCardError> {
        if val >= 2 && val <= 53 {
            Ok(Self(NonZeroU8::new(val).unwrap()))
        } else {
            Err(PlayingCardError::InvalidValue)
        }
    }

    // PlayingCard -> u8: {2, ..., 53}
    pub fn val(&self) -> NonZeroU8 {
        self.0
    }

    /// PlayingCard -> Rank: {2, ..., 14}
    pub fn rank(&self) -> Rank {
        let r: u8 = (self.0.get() - 2) % 13;
        Rank::try_from(r + 2).unwrap()
    }

    /// PlayingCard -> Suit: {0, ..., 3}
    pub fn suit(&self) -> Suit {
        let s: u8 = (self.0.get() - 2) / 13;
        Suit::try_from(s).unwrap()
    }
}

impl FromStr for PlayingCard {
    type Err = PlayingCardError;

    /// &str: "{rank} {suit}" -> Playingcard
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.to_lowercase();
        let a: Vec<&str> = s.split_whitespace().collect();
        if a.len() != 2 {
            Err(PlayingCardError::InvalidStringFormat)
        } else {
            Ok(PlayingCard::new(
                Rank::from_str(a[0])?,
                Suit::from_str(a[1])?,
            ))
        }
    }
}

impl Display for PlayingCard {
    /// PlayingCard -> &str: "{rank} {suit}"
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.rank(), self.suit())
    }
}

impl Debug for PlayingCard {
    /// PlayingCard -> Display trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Rank: {2=2, ..., Kind=13, Ace=14}
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

impl TryFrom<u8> for Rank {
    type Error = RankError;

    /// u8: {2, ..., 14} -> Rank
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Jack),
            12 => Ok(Self::Queen),
            13 => Ok(Self::King),
            14 => Ok(Self::Ace),
            _ => Err(RankError),
        }
    }
}

impl FromStr for Rank {
    type Err = RankError;

    /// &str: {2, ..., 10, jack, queen, king, ace} -> Rank
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
            _ => Err(RankError),
        }
    }
}

impl Display for Rank {
    /// Rank -> &str: {2, ..., 10, jack, queen, king, ace}
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "jack",
            Self::Queen => "queen",
            Self::King => "king",
            Self::Ace => "ace",
        };

        (&mut *f).write_str(s)
    }
}

impl Debug for Rank {
    /// Rank -> Display trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Suit: {Heart=0, Diamond=1, Club=2, Spade=3}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Suit {
    Heart = 0,
    Diamond = 1,
    Club = 2,
    Spade = 3,
}

impl TryFrom<u8> for Suit {
    type Error = SuitError;

    /// u8: {0, 1, 2, 3} -> Suit
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(Self::Heart),
            1 => Ok(Self::Diamond),
            2 => Ok(Self::Club),
            3 => Ok(Self::Spade),
            _ => Err(SuitError),
        }
    }
}

impl FromStr for Suit {
    type Err = SuitError;

    /// &str: {heart, diamond, club, spade} -> Suit
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "heart" => Ok(Self::Heart),
            "diamond" => Ok(Self::Diamond),
            "club" => Ok(Self::Club),
            "spade" => Ok(Self::Spade),
            _ => Err(SuitError),
        }
    }
}

impl Display for Suit {
    /// Suit -> &str: {heart, diamond, club, spade}
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = match self {
            Self::Heart => "heart",
            Self::Diamond => "diamond",
            Self::Club => "club",
            Self::Spade => "spade",
        };

        (&mut *f).write_str(s)
    }
}

impl Debug for Suit {
    /// Suit -> Display trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// All Rank variants in ascending order (Rank::Two to Rank::Ace)
pub const RANKS: [Rank; 13] = [
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
];

/// All Suit variants
pub const SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];

/// Creates a deck of cards by taking the cartesian product of the given suits and ranks (all arrays)
const fn cartesian_deck<const M: usize, const N: usize, const K: usize>(
    suits: [Suit; M],
    ranks: [Rank; N],
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

/// Entire standard playing card deck (52 cards: all suits and ranks from 2 to Ace)
pub const DECK_52: [PlayingCard; RANKS.len() * SUITS.len()] = cartesian_deck(SUITS, RANKS);

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn cartesian_deck_small() {
        let suits: [Suit; 2] = [Suit::Diamond, Suit::Club];
        let ranks: [Rank; 3] = [Rank::Two, Rank::Ace, Rank::Ten];
        let result: [PlayingCard; 6] = cartesian_deck(suits, ranks);
        assert_eq!(
            result,
            [
                PlayingCard::new(Rank::Two, Suit::Diamond),
                PlayingCard::new(Rank::Ace, Suit::Diamond),
                PlayingCard::new(Rank::Ten, Suit::Diamond),
                PlayingCard::new(Rank::Two, Suit::Club),
                PlayingCard::new(Rank::Ace, Suit::Club),
                PlayingCard::new(Rank::Ten, Suit::Club),
            ]
        );
    }

    #[test]
    fn compare_ranks() {
        assert_eq!(Rank::Two.cmp(&Rank::Two), Ordering::Equal);
        assert_eq!(Rank::Two.cmp(&Rank::Three), Ordering::Less);
        assert_eq!(Rank::Two.cmp(&Rank::Ace), Ordering::Less);
        assert_eq!(Rank::Three.cmp(&Rank::Two), Ordering::Greater);
        assert_eq!(Rank::Three.cmp(&Rank::Ace), Ordering::Less);
        assert_eq!(Rank::Ace.cmp(&Rank::Two), Ordering::Greater);
        assert_eq!(Rank::Ace.cmp(&Rank::King), Ordering::Greater);
    }

    #[test]
    fn suit_equality() {
        assert_eq!(Suit::Club == Suit::Club, true);
        assert_eq!(Suit::Club == Suit::Diamond, false);
        assert_eq!(Suit::Club == Suit::Heart, false);
        assert_eq!(Suit::Club == Suit::Spade, false);
    }

    #[test]
    fn from() {
        assert_eq!(Suit::from_str("club"), Ok(Suit::Club));
        assert_eq!(Suit::from_str("hear"), Err(SuitError));
        assert_eq!(Rank::from_str("2"), Ok(Rank::Two));
        assert_eq!(Rank::from_str("ac"), Err(RankError));
        assert_eq!(Suit::try_from(0x00), Ok(Suit::Heart));
        assert_eq!(Suit::try_from(0x01), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from(0x02), Ok(Suit::Club));
        assert_eq!(Suit::try_from(0x03), Ok(Suit::Spade));
        assert_eq!(Rank::try_from(0x02), Ok(Rank::Two));
        assert_eq!(Rank::try_from(0x03), Ok(Rank::Three));
        assert_eq!(Rank::try_from(0x04), Ok(Rank::Four));
        assert_eq!(Rank::try_from(0x05), Ok(Rank::Five));
        assert_eq!(
            PlayingCard::from_val(53),
            Ok(PlayingCard::new(Rank::Ace, Suit::Spade))
        );
        assert_eq!(
            PlayingCard::from_val(2),
            Ok(PlayingCard::new(Rank::Two, Suit::Heart))
        );
        assert_eq!(
            PlayingCard::from_val(54),
            Err(PlayingCardError::InvalidValue)
        );
        assert_eq!(
            PlayingCard::from_val(1),
            Err(PlayingCardError::InvalidValue)
        );
    }
}
