use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Thrown if `Suit::try_from<u8>` fails.
#[derive(PartialEq, Debug)]
pub struct SuitValueError {
    bad_value: u8,
}

impl SuitValueError {
    pub fn new(bad_value: u8) -> Self {
        Self { bad_value }
    }
}

impl std::error::Error for SuitValueError {}

impl Display for SuitValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Thrown if `Suit::from_str` fails.
#[derive(PartialEq, Debug)]
pub struct SuitStringError {
    bad_string: String,
}

impl SuitStringError {
    pub fn new(bad_string: String) -> Self {
        Self { bad_string }
    }
}

impl std::error::Error for SuitStringError {}

impl Display for SuitStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
    /// All Suit variants.
    /// Don't rely on the order.
    pub const ALL: [Self; 4] = [Self::Heart, Self::Diamond, Self::Club, Self::Spade];
}

impl TryFrom<u8> for Suit {
    type Error = SuitValueError;

    /// u8: {0, 1, 2, 3} -> Suit
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(Self::Heart),
            1 => Ok(Self::Diamond),
            2 => Ok(Self::Club),
            3 => Ok(Self::Spade),
            v => Err(SuitValueError::new(v)),
        }
    }
}

impl FromStr for Suit {
    type Err = SuitStringError;

    /// &str: {heart, diamond, club, spade} -> Suit
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_ref() {
            "heart" => Ok(Self::Heart),
            "diamond" => Ok(Self::Diamond),
            "club" => Ok(Self::Club),
            "spade" => Ok(Self::Spade),
            _ => Err(SuitStringError::new(s)),
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

#[cfg(feature = "serde")]
mod serde_impls {
    use super::Suit;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

    impl Serialize for Suit {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.collect_str(self) // via Display: "heart"
        }
    }

    impl<'de> Deserialize<'de> for Suit {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            String::deserialize(d)?.parse().map_err(de::Error::custom) // via FromStr, so validated
        }
    }
}

#[cfg(test)]
mod tests {
    //! Tests the suit module.

    use super::*;

    /// Tests the `from_str` and `try_from` functions.
    #[test]
    fn from_str_u8() {
        assert_eq!(Suit::from_str("club"), Ok(Suit::Club));
        assert_eq!(
            Suit::from_str("hear"),
            Err(SuitStringError::new("hear".to_string()))
        );
        assert_eq!(Suit::try_from(0), Ok(Suit::Heart));
        assert_eq!(Suit::try_from(1), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from(2), Ok(Suit::Club));
        assert_eq!(Suit::try_from(3), Ok(Suit::Spade));
    }
}
