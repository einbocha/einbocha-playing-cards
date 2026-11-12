#![warn(unreachable_pub)]

mod card_set;
mod deck;
mod playing_card;

#[cfg(feature = "schema")]
pub use schemars;

pub use card_set::{CardSet, CardSetIter};
pub use deck::{DECK_52, cartesian_deck};
pub use playing_card::rank::{Rank, RankStringError, RankValueError};
pub use playing_card::suit::{Suit, SuitStringError, SuitValueError};
pub use playing_card::{PlayingCard, PlayingCardStringError, PlayingCardValueError};
