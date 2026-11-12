use crate::playing_card::PlayingCard;
use crate::playing_card::rank::Rank;
use crate::playing_card::suit::Suit;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// Memory efficient HashSet like datastructure for fast existence checking, adding and removing
/// It can keep track of any PlayingCard (once as it resembles a set).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CardSet(u64);

impl Default for CardSet {
    fn default() -> Self {
        Self::new()
    }
}

impl CardSet {
    /// Creates an empty CardSet
    #[inline(always)]
    pub const fn new() -> Self {
        Self::empty()
    }

    /// Creates an empty CardSet
    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Creates a full CardSet containing all 52 Cards
    #[inline(always)]
    pub const fn full() -> Self {
        Self(Self::mask_all())
    }

    /// Creates a CardSet from a u64 according to its internal binary format.
    /// Only use if really necessary.
    #[inline(always)]
    pub const fn from_val(val: u64) -> Self {
        Self(val & Self::mask_all())
    }

    /// Returns the internal bit representation.
    #[inline(always)]
    pub const fn val(&self) -> u64 {
        self.0
    }

    /// Adds a card to the set, returns true on addition of the card, false if the card was already present
    #[inline(always)]
    pub const fn add(&mut self, c: PlayingCard) -> bool {
        let old: u64 = self.0;
        self.0 |= 1_u64 << c.val().get();
        self.0 != old // The card was only added if the state has changed
    }

    /// Removes a card from the set, returns true on removal of the card, false if the card was not present
    #[inline(always)]
    pub const fn remove(&mut self, c: PlayingCard) -> bool {
        let old: u64 = self.0;
        self.0 &= !(1_u64 << c.val().get());
        self.0 != old // The card was only removed if the state has changed
    }

    /// Removes all cards from the set, returns true if at least one cards is removed else false.
    #[inline(always)]
    pub const fn clear(&mut self) -> bool {
        let old: u64 = self.0;
        self.0 = 0;
        self.0 != old // The Card set wasn't already empty
    }

    /// Pops a card from the set and returns it.
    /// If the set is empty None is returned.
    #[inline(always)]
    pub fn pop_any(&mut self) -> Option<PlayingCard> {
        if self.0 == 0 {
            return None;
        }

        // position of lowest bit / card
        let idx = self.0.trailing_zeros();
        // clear lowest 1-bit (i.e. remove the respecive card): Brian Kernighan's algorithm
        self.0 &= self.0 - 1;

        Some(PlayingCard::from_val(idx as u8).unwrap()) // as the set is not empty there has to be a valid card
    }

    /// Checks if the card is present in the set
    #[inline(always)]
    pub const fn contains(&self, c: PlayingCard) -> bool {
        self.0 & (1_u64 << c.val().get()) != 0
    }

    /// Checks if the set is empty
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Counts the cards in the set.
    #[inline(always)]
    pub const fn len(&self) -> u32 {
        self.0.count_ones()
    }

    /// ANDs the mask with the CardSet. Every card with a 1 in the mask is kept.
    #[inline(always)]
    pub const fn filter_by_mask(&mut self, mask: u64) {
        self.0 &= mask;
    }

    /// Masks all 13 cards from a suit
    #[inline(always)]
    pub const fn mask_suit(suit: Suit) -> u64 {
        ((1_u64 << 13) - 1) << (suit as u8 * 16 + 2)
    }

    /// Masks all 4 cards with the same rank
    #[inline(always)]
    pub const fn mask_rank(rank: Rank) -> u64 {
        ((1_u64 << 2) | (1_u64 << 18) | (1_u64 << 34) | (1_u64 << 50)) << (rank as u8 - 2)
    }

    /// Masks all 52 cards
    #[inline(always)]
    pub const fn mask_all() -> u64 {
        Self::mask_suit(Suit::Heart)
            | Self::mask_suit(Suit::Diamond)
            | Self::mask_suit(Suit::Club)
            | Self::mask_suit(Suit::Spade)
    }

    /// Creates an iterator over the set without consuming the set.
    #[inline(always)]
    pub const fn iter(&self) -> CardSetIter {
        CardSetIter(self.0)
    }
}

impl BitAnd for CardSet {
    type Output = Self;

    /// Creates a new CardSet containing the cards from the mathematical intersection of the two provided CardSets.
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for CardSet {
    /// Keeps only the cards that are in both sets (in place).
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for CardSet {
    type Output = Self;

    /// Creates a new CardSet containing the cards from the mathematical union of the two provided CardSets.
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CardSet {
    /// Adds all cards from the other set that are not yet contained (in place).
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for CardSet {
    type Output = Self;

    /// Creates a new CardSet containing the cards form the mathematical expression: union set-minus intersection.
    /// In other words only cards that are exactly in one of both sets are put into the new set.
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for CardSet {
    /// Cards that are in the other set are removed. Cards in the other set that are new, are added.
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for CardSet {
    type Output = Self;

    /// Creates a new CardSet containing exactly
    #[inline(always)]
    fn not(self) -> Self::Output {
        // We use from_val to make sure that no unused bit flips to 1.
        Self::from_val(!self.0)
    }
}

/// Helper struct to implement an iterator for CardSet.
pub struct CardSetIter(u64);

impl Iterator for CardSetIter {
    type Item = PlayingCard;

    /// Returns cards one by one starting with 'Two of Hearts' up to 'Ace of Hearts' and then all Diamond, Club, Spade cards.
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        // position of lowest bit / card
        let idx = self.0.trailing_zeros();
        // clear lowest 1-bit (i.e. remove the respecive card): Brian Kernighan's algorithm
        self.0 &= self.0 - 1;

        Some(PlayingCard::from_val(idx as u8).unwrap()) // as the set is not empty there has to be a valid card
    }

    /// Returns the exact number of cards in the iterator (lower_bound, upper_bound).
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.0.count_ones() as usize;
        (count, Some(count))
    }
}

impl IntoIterator for CardSet {
    type Item = PlayingCard;
    type IntoIter = CardSetIter;

    /// Consumes a CardSet and returns a CardSet-Iterator.
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        CardSetIter(self.0)
    }
}

#[cfg(test)]
mod tests {
    //! Tests for the card_set module.

    use super::*;

    /// Tests the card set iterator.
    #[test]
    fn card_set_iterator() {
        let set: CardSet = CardSet::full();
        let mut set_iter: CardSetIter = set.iter();
        assert_eq!(
            set_iter.next(),
            Some(PlayingCard::new(Rank::Two, Suit::Heart))
        );
        assert_eq!(
            set_iter.next(),
            Some(PlayingCard::new(Rank::Three, Suit::Heart))
        );
        assert_eq!(set_iter.size_hint(), (50_usize, Some(50_usize)));
    }

    #[test]
    fn card_set_pop_any() {
        let mut a = CardSet::empty();
        assert_eq!(a.pop_any(), None);
        a.add(PlayingCard::new(Rank::Ace, Suit::Club));
        assert_eq!(a.pop_any(), Some(PlayingCard::new(Rank::Ace, Suit::Club)));
        assert_eq!(a, CardSet::empty());
    }

    #[test]
    fn card_set_masking() {
        let mut set: CardSet = CardSet::full();
        set.filter_by_mask(CardSet::mask_all());
        assert_eq!(set.len(), 52);

        for suit in Suit::ALL {
            let mut set: CardSet = CardSet::full();
            set.filter_by_mask(CardSet::mask_suit(suit));
            assert_eq!(set.len(), 13);
            for rank in Rank::ALL {
                assert!(set.contains(PlayingCard::new(rank, suit)))
            }
        }

        for rank in Rank::ALL {
            let mut set: CardSet = CardSet::full();
            set.filter_by_mask(CardSet::mask_rank(rank));
            assert_eq!(set.len(), 4);
            for suit in Suit::ALL {
                assert!(set.contains(PlayingCard::new(rank, suit)));
            }
        }
    }

    #[test]
    fn card_set_bin_ops() {
        let mut a: CardSet = CardSet::new();
        a.add(PlayingCard::new(Rank::Ace, Suit::Club));
        a.add(PlayingCard::new(Rank::Two, Suit::Club));

        let mut b: CardSet = CardSet::new();
        b.add(PlayingCard::new(Rank::Ace, Suit::Club));
        b.add(PlayingCard::new(Rank::Three, Suit::Club));

        // intersection
        let intersection_ab: CardSet = a & b;
        let mut sol: CardSet = CardSet::new();
        sol.add(PlayingCard::new(Rank::Ace, Suit::Club));
        assert_eq!(intersection_ab, sol);

        // union
        let union_ab: CardSet = a | b;
        let mut sol: CardSet = CardSet::new();
        sol.add(PlayingCard::new(Rank::Ace, Suit::Club));
        sol.add(PlayingCard::new(Rank::Two, Suit::Club));
        sol.add(PlayingCard::new(Rank::Three, Suit::Club));
        assert_eq!(union_ab, sol);

        // xor
        let xor_ab: CardSet = a ^ b;
        sol.clear();
        sol.add(PlayingCard::new(Rank::Two, Suit::Club));
        sol.add(PlayingCard::new(Rank::Three, Suit::Club));
        assert_eq!(xor_ab, sol);

        //not
        let not_a = !a;
        let mut sol = CardSet::full();
        sol.remove(PlayingCard::new(Rank::Ace, Suit::Club));
        sol.remove(PlayingCard::new(Rank::Two, Suit::Club));
        assert_eq!(not_a, sol);
    }

    #[test]
    fn card_set_clear_full_count() {
        let mut set: CardSet = CardSet::full();
        assert_eq!(set.len(), 52);
        set.clear();
        assert_eq!(set, CardSet::empty());
    }

    /// Tests the `is_empty`, `contains`, `remove`, `add` functions against a single `PlayingCard`.
    #[test]
    fn check_empty_add_remove_contains() {
        let mut card_set: CardSet = CardSet::new();
        let card: PlayingCard = PlayingCard::from_val(2).unwrap();
        assert!(card_set.is_empty());
        assert!(!card_set.contains(card));
        assert!(!card_set.remove(card));
        assert!(card_set.add(card));
        assert!(!card_set.add(card));
        assert!(!card_set.is_empty());
        assert!(card_set.contains(card));
        assert!(card_set.remove(card));
    }

    /// Checks the `filter_by_mask` function with using the `mask_rank` and `mask_suit` masks.
    #[test]
    fn filter_cardset() {
        let king: PlayingCard = PlayingCard::new(Rank::King, Suit::Club);
        let ace: PlayingCard = PlayingCard::new(Rank::Ace, Suit::Club);
        let ten: PlayingCard = PlayingCard::new(Rank::Ten, Suit::Club);
        let spade: PlayingCard = PlayingCard::new(Rank::Two, Suit::Spade);
        let diamond: PlayingCard = PlayingCard::new(Rank::Two, Suit::Diamond);

        let mut a: CardSet = CardSet::new();
        a.add(king);
        a.add(ace);
        a.add(ten);
        a.add(spade);
        a.add(diamond);

        let mut b: CardSet = a; // copies `CardSet` a

        a.filter_by_mask(CardSet::mask_suit(Suit::Club));

        assert!(a.contains(king));
        assert!(a.contains(ace));
        assert!(a.contains(ten));

        assert!(!a.contains(spade));
        assert!(!a.contains(diamond));

        b.filter_by_mask(CardSet::mask_rank(Rank::Two));

        assert!(b.contains(spade));
        assert!(b.contains(diamond));

        assert!(!b.contains(king));
        assert!(!b.contains(ace));
        assert!(!b.contains(ten));
    }
}
