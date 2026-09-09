use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    num::NonZeroU8,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    str::FromStr,
};

/// Thrown if no Rank can be derived
#[derive(PartialEq, Debug)]
pub enum RankError {
    InvalidString,
    InvalidValue,
}

impl std::error::Error for RankError {}

impl Display for RankError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Thrown if no Suit can be derived
#[derive(PartialEq, Debug)]
pub enum SuitError {
    InvalidString,
    InvalidValue,
}

impl std::error::Error for SuitError {}

impl Display for SuitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Thrown if no PlayingCard can be derived
#[derive(PartialEq, Debug)]
pub enum PlayingCardError {
    InvalidStringFormat,
    InvalidValue,
}

impl std::error::Error for PlayingCardError {}

impl Display for PlayingCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<RankError> for PlayingCardError {
    fn from(e: RankError) -> Self {
        match e {
            RankError::InvalidString => Self::InvalidStringFormat,
            RankError::InvalidValue => Self::InvalidValue,
        }
    }
}

impl From<SuitError> for PlayingCardError {
    fn from(e: SuitError) -> Self {
        match e {
            SuitError::InvalidString => Self::InvalidStringFormat,
            SuitError::InvalidValue => Self::InvalidValue,
        }
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
    /// Performance: fast (SHIFT, OR operations)
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let packed: u8 = ((suit as u8) << 4) | (rank as u8);
        Self(NonZeroU8::new(packed).unwrap())
    }

    /// u8: {2, ..., 14, 18, ..., 30, 34, ..., 46, 50, ..., 62} -> PlayingCard
    /// Performance: fast (AND, SHIFT operations)
    pub fn from_val(val: u8) -> Result<Self, PlayingCardError> {
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
    type Err = PlayingCardError;

    /// &str: "{rank} {suit}" -> Playingcard
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.to_lowercase();
        let mut parts = s.split_whitespace();

        let rank_str: &str = parts.next().ok_or(PlayingCardError::InvalidStringFormat)?;
        let suit_str: &str = parts.next().ok_or(PlayingCardError::InvalidStringFormat)?;

        if parts.next().is_some() {
            return Err(PlayingCardError::InvalidStringFormat);
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

/// Rank: {2=2, ..., King=13, Ace=14}
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
    // All Rank variants in ascending order (Rank::Two to Rank::Ace)
    pub const ALL: [Self; 13] = [
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];
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
            _ => Err(RankError::InvalidValue),
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
            _ => Err(RankError::InvalidString),
        }
    }
}

impl Display for Rank {
    /// Rank -> &str: {2, ..., 10, jack, queen, king, ace}
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
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
        })
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

impl Suit {
    /// All Suit variants
    pub const ALL: [Self; 4] = [Self::Heart, Self::Diamond, Self::Club, Self::Spade];
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
            _ => Err(SuitError::InvalidValue),
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
            _ => Err(SuitError::InvalidString),
        }
    }
}

impl Display for Suit {
    /// Suit -> &str: {heart, diamond, club, spade}
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Heart => "heart",
            Self::Diamond => "diamond",
            Self::Club => "club",
            Self::Spade => "spade",
        })
    }
}

impl Debug for Suit {
    /// Suit -> Display trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Creates a deck of cards by taking the cartesian product of the given suits and ranks (all arrays)
pub const fn cartesian_deck<const M: usize, const N: usize, const K: usize>(
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
pub const DECK_52: [PlayingCard; Rank::ALL.len() * Suit::ALL.len()] =
    cartesian_deck(Suit::ALL, Rank::ALL);

/// Memory efficient HashSet like datastructure for fast existence checking, adding and removing
/// It can keep track of any PlayingCard (once as it resembles a set).
#[derive(Debug, Copy, Clone)]
pub struct CardSet(u64);

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

    /// Pops a card from the set and returns it.
    /// If the set is empty None is returned.
    #[inline(always)]
    pub fn pop_any(&mut self) -> Option<PlayingCard> {
        let idx: u8 = self.0.trailing_zeros() as u8;
        PlayingCard::from_val(idx).ok()
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
    pub const fn count_cards(&self) -> u32 {
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
        (1_u64 << 13) - 1 << (suit as u8 * 16 + 2)
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
        assert_eq!(Suit::from_str("hear"), Err(SuitError::InvalidString));
        assert_eq!(Rank::from_str("2"), Ok(Rank::Two));
        assert_eq!(Rank::from_str("ac"), Err(RankError::InvalidString));
        assert_eq!(Suit::try_from(0x00), Ok(Suit::Heart));
        assert_eq!(Suit::try_from(0x01), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from(0x02), Ok(Suit::Club));
        assert_eq!(Suit::try_from(0x03), Ok(Suit::Spade));
        assert_eq!(Rank::try_from(0x02), Ok(Rank::Two));
        assert_eq!(Rank::try_from(0x03), Ok(Rank::Three));
        assert_eq!(Rank::try_from(0x04), Ok(Rank::Four));
        assert_eq!(Rank::try_from(0x05), Ok(Rank::Five));
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
            Err(PlayingCardError::InvalidValue)
        );
        assert_eq!(
            PlayingCard::from_val(1),
            Err(PlayingCardError::InvalidValue)
        );

        assert_eq!(PlayingCard::new(Rank::Two, Suit::Heart).val().get(), 2);
    }

    #[test]
    fn card_set() {
        let mut card_set: CardSet = CardSet::new();
        let card: PlayingCard = PlayingCard::from_val(2).unwrap();
        assert_eq!(card_set.is_empty(), true);
        assert_eq!(card_set.contains(card), false);
        assert_eq!(card_set.remove(card), false);
        assert_eq!(card_set.add(card), true);
        assert_eq!(card_set.add(card), false);
        assert_eq!(card_set.is_empty(), false);
        assert_eq!(card_set.contains(card), true);
        assert_eq!(card_set.remove(card), true);
    }

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
        let mut b: CardSet = a;
        a.filter_by_mask(CardSet::mask_suit(Suit::Club));
        assert_eq!(a.contains(king), true);
        assert_eq!(a.contains(ace), true);
        assert_eq!(a.contains(ten), true);

        assert_eq!(a.contains(spade), false);
        assert_eq!(a.contains(diamond), false);

        b.filter_by_mask(CardSet::mask_rank(Rank::Two));
        assert_eq!(b.contains(spade), true);
        assert_eq!(b.contains(diamond), true);

        assert_eq!(b.contains(king), false);
        assert_eq!(b.contains(ace), false);
        assert_eq!(b.contains(ten), false);
    }
}
