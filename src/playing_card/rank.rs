use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Thrown if `Rank::try_from<u8>` fails.
#[derive(PartialEq, Debug)]
pub struct RankValueError {
    bad_value: u8,
}

impl RankValueError {
    pub fn new(bad_value: u8) -> Self {
        Self { bad_value }
    }
}

impl std::error::Error for RankValueError {}

impl Display for RankValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Thrown if `Rank::from_str` fails.
#[derive(PartialEq, Debug)]
pub struct RankStringError {
    bad_string: String,
}

impl RankStringError {
    pub fn new(bad_string: String) -> Self {
        Self { bad_string }
    }
}

impl std::error::Error for RankStringError {}

impl Display for RankStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Rank: {2=2, ..., King=13, Ace=14}
#[derive(PartialOrd, PartialEq, Clone, Copy, Ord, Eq, Hash)]
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
    type Error = RankValueError;

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
            v => Err(RankValueError::new(v)),
        }
    }
}

impl FromStr for Rank {
    type Err = RankStringError;

    /// &str: {2, ..., 10, jack, queen, king, ace} -> Rank
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_ref() {
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
            _ => Err(RankStringError::new(s)),
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

#[cfg(feature = "serde")]
mod serde_impls {
    use super::Rank;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

    impl Serialize for Rank {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.collect_str(self) // via Display: "ace"
        }
    }

    impl<'de> Deserialize<'de> for Rank {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            String::deserialize(d)?.parse().map_err(de::Error::custom) // via FromStr, so validated
        }
    }
}

#[cfg(test)]
mod tests {
    //! Tests the rank module.

    use super::*;
    use std::cmp::Ordering;

    /// Checks the implicitly defined ordering of different ranks.
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

    /// Tests the `from_str` and `try_from` functions.
    #[test]
    fn from_str_u8() {
        assert_eq!(Rank::from_str("2"), Ok(Rank::Two));
        assert_eq!(
            Rank::from_str("ac"),
            Err(RankStringError::new("ac".to_string()))
        );
        assert_eq!(Rank::try_from(2), Ok(Rank::Two));
        assert_eq!(Rank::try_from(3), Ok(Rank::Three));
        assert_eq!(Rank::try_from(4), Ok(Rank::Four));
        assert_eq!(Rank::try_from(5), Ok(Rank::Five));
    }
}
