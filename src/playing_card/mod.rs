pub(crate) mod rank;
pub(crate) mod suit;

use rank::{Rank, RankStringError, RankValueError};
use suit::{Suit, SuitStringError, SuitValueError};

use std::fmt::{Debug, Display, Formatter};
use std::num::NonZeroU8;
use std::str::FromStr;

/// Thrown if `PlayingCard::from_str` fails.
#[derive(PartialEq, Debug)]
pub enum PlayingCardStringError {
    InvalidRank(RankStringError),
    InvalidSuit(SuitStringError),
    InvalidFormat,
}

impl std::error::Error for PlayingCardStringError {}

impl Display for PlayingCardStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<RankStringError> for PlayingCardStringError {
    fn from(e: RankStringError) -> Self {
        Self::InvalidRank(e)
    }
}

impl From<SuitStringError> for PlayingCardStringError {
    fn from(e: SuitStringError) -> Self {
        Self::InvalidSuit(e)
    }
}

/// Thrown if `PlayingCard::from_val` fails.
#[derive(PartialEq, Debug)]
pub enum PlayingCardValueError {
    InvalidRank(RankValueError),
    InvalidSuit(SuitValueError),
}

impl std::error::Error for PlayingCardValueError {}

impl Display for PlayingCardValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<RankValueError> for PlayingCardValueError {
    fn from(e: RankValueError) -> Self {
        Self::InvalidRank(e)
    }
}

impl From<SuitValueError> for PlayingCardValueError {
    fn from(e: SuitValueError) -> Self {
        Self::InvalidSuit(e)
    }
}

/// Highly performant playing card representation.
/// Stored as a single byte in memory.
/// Wrapping a card in an Option<> doesn't introduce any overhead
/// as the card can be encoded internally using a NonZeroU8.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlayingCard(NonZeroU8);

impl PlayingCard {
    /// Returns guaranteed a valid PlayingCard
    /// Performance: fast (SHIFT, OR operations)
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let packed: u8 = ((suit as u8) << 4) | (rank as u8);
        Self(NonZeroU8::new(packed).unwrap())
    }

    /// u8: {2, ..., 14, 18, ..., 30, 34, ..., 46, 50, ..., 62} -> PlayingCard
    /// Performance: fast (AND, SHIFT operations)
    pub fn from_val(val: u8) -> Result<Self, PlayingCardValueError> {
        let _: Rank = Rank::try_from(val & 0x0F)?;
        let _: Suit = Suit::try_from(val >> 4)?;
        Ok(Self(NonZeroU8::new(val).unwrap()))
    }

    /// PlayingCard -> u8: {2, ..., 14, 18, ..., 30, 34, ..., 46, 50, ..., 62}
    /// Performance: fast (no operations)
    pub const fn val(&self) -> NonZeroU8 {
        self.0
    }

    /// PlayingCard -> Rank: {2, ..., 14}
    /// Performance: fast (AND operation)
    pub fn rank(&self) -> Rank {
        Rank::try_from(self.0.get() & 0x0F).unwrap()
    }

    /// PlayingCard -> Suit: {0, ..., 3}
    /// Performance: fast (SHIFT operation)
    pub fn suit(&self) -> Suit {
        Suit::try_from(self.0.get() >> 4).unwrap()
    }
}

impl FromStr for PlayingCard {
    type Err = PlayingCardStringError;

    /// &str: "{rank} {suit}" -> Playingcard
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.to_lowercase();
        let mut parts = s.split_whitespace();

        let rank_str: &str = parts.next().ok_or(PlayingCardStringError::InvalidFormat)?;
        let suit_str: &str = parts.next().ok_or(PlayingCardStringError::InvalidFormat)?;

        if parts.next().is_some() {
            return Err(PlayingCardStringError::InvalidFormat);
        }

        Ok(PlayingCard::new(
            Rank::from_str(rank_str)?,
            Suit::from_str(suit_str)?,
        ))
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

#[cfg(test)]
mod tests {
    //! Tests the playing_card module.

    use super::*;

    /// Tests `from_str` and its consistency with the `Display` implementation for `PlayingCard`.
    #[test]
    fn from_str() {
        let strings = [
            "2 heart",
            "ace club",
            "kin diamond",
            "king spades",
            "King SpaDe",
        ];

        let results = [
            Ok(PlayingCard::new(Rank::Two, Suit::Heart)),
            Ok(PlayingCard::new(Rank::Ace, Suit::Club)),
            Err(PlayingCardStringError::InvalidRank(RankStringError::new(
                "kin".to_string(),
            ))),
            Err(PlayingCardStringError::InvalidSuit(SuitStringError::new(
                "spades".to_string(),
            ))),
            Ok(PlayingCard::new(Rank::King, Suit::Spade)),
        ];

        for (s, res) in strings.iter().zip(results.iter()) {
            assert_eq!(&PlayingCard::from_str(s), res);
        }
    }

    /// Tests `from_val`.
    #[test]
    fn from_val() {
        assert_eq!(
            PlayingCard::from_val(62),
            Ok(PlayingCard::new(Rank::Ace, Suit::Spade))
        );
        assert_eq!(
            PlayingCard::from_val(2),
            Ok(PlayingCard::new(Rank::Two, Suit::Heart))
        );
        assert_eq!(
            PlayingCard::from_val(49),
            Err(PlayingCardValueError::InvalidRank(RankValueError::new(1)))
        );
        assert_eq!(
            PlayingCard::from_val(1),
            Err(PlayingCardValueError::InvalidRank(RankValueError::new(1)))
        );

        assert_eq!(PlayingCard::new(Rank::Two, Suit::Heart).val().get(), 2);
    }
}
