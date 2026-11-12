use crate::playing_card::PlayingCard;
use crate::playing_card::rank::Rank;
use crate::playing_card::suit::Suit;

/// Entire standard playing card deck (52 cards: all suits and ranks from 2 to Ace)
pub const DECK_52: [PlayingCard; Rank::ALL.len() * Suit::ALL.len()] =
    cartesian_deck(Suit::ALL, Rank::ALL);

/// Creates a deck of cards by taking the cartesian product of the given suits and ranks (all arrays)
pub const fn cartesian_deck<const M: usize, const N: usize, const K: usize>(
    suits: [Suit; M],
    ranks: [Rank; N],
) -> [PlayingCard; K] {
    assert!(K == M * N, "M * N == K must hold");

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

#[cfg(test)]
mod tests {
    //! Tests the deck module.

    use super::*;

    /// Tests the cartesian set constructor function.
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
}
