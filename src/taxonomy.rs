use std::fmt::Display;

use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use std::sync::Mutex;
use tracing::instrument;

use crate::errors::TaxRankParsingError;

static LAST_TAXONOMY_RANK_PARSED: Lazy<Mutex<Option<Rank>>> = Lazy::new(|| Mutex::new(None));

/// Taxonomy levels
///
/// the u32 offset represents sub-clade (e.g. parvorder, subfamily, etc.)
#[derive(Clone, PartialEq, Debug, PartialOrd, Ord, Eq, Hash, Copy)]
pub enum Rank {
    Unclassified(u32),
    Root(u32),
    Domain(u32),
    Kingdom(u32),
    Phylum(u32),
    Class(u32),
    Order(u32),
    Family(u32),
    Genus(u32),
    Species(u32),
}

// TODO: order D1 as below of any R0..9
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unclassified(i) => write!(f, "U{}", i),
            Self::Root(i) => write!(f, "R{}", i),
            Self::Domain(i) => write!(f, "D{}", i),
            Self::Kingdom(i) => write!(f, "K{}", i),
            Self::Phylum(i) => write!(f, "P{}", i),
            Self::Class(i) => write!(f, "C{}", i),
            Self::Order(i) => write!(f, "O{}", i),
            Self::Family(i) => write!(f, "F{}", i),
            Self::Genus(i) => write!(f, "G{}", i),
            Self::Species(i) => write!(f, "S{}", i),
        }
    }
}

impl Rank {
    #[must_use]
    pub fn plus_one(self) -> Self {
        match self {
            Self::Unclassified(i) => Self::Unclassified(i.checked_add(1).unwrap()),
            Self::Root(i) => Self::Root(i.checked_add(1).unwrap()),
            Self::Domain(i) => Self::Domain(i.checked_add(1).unwrap()),
            Self::Kingdom(i) => Self::Kingdom(i.checked_add(1).unwrap()),
            Self::Phylum(i) => Self::Phylum(i.checked_add(1).unwrap()),
            Self::Class(i) => Self::Class(i.checked_add(1).unwrap()),
            Self::Order(i) => Self::Order(i.checked_add(1).unwrap()),
            Self::Family(i) => Self::Family(i.checked_add(1).unwrap()),
            Self::Genus(i) => Self::Genus(i.checked_add(1).unwrap()),
            Self::Species(i) => Self::Species(i.checked_add(1).unwrap()),
        }
    }
}

#[instrument]
pub fn parse_taxonomy_level(string: &str) -> Result<Rank, TaxRankParsingError> {
    // TODO: add previous tax rank here, make it purely functional
    if string.len() > 2 {
        return Err(TaxRankParsingError::InvalidLength(
            String::from(string),
            string.len(),
        ));
    }

    let mut string_chars = string.chars();

    let letter = string_chars.next().unwrap();

    let offset: u32 =
        string_chars
            .next()
            .map_or(Ok(0_u32), |number| -> Result<u32, TaxRankParsingError> {
                if number.is_ascii_digit() {
                    Ok(number.to_digit(10_u32).unwrap())
                } else {
                    Err(TaxRankParsingError::OffsetNotANumber(
                        String::from(string),
                        number,
                    ))
                }
            })?;

    let tax_rank: Rank = match letter {
        'U' => Ok(Rank::Unclassified(offset)),
        'R' => Ok(Rank::Root(offset)),
        'D' => Ok(Rank::Domain(offset)),
        'K' => Ok(Rank::Kingdom(offset)),
        'P' => Ok(Rank::Phylum(offset)),
        'C' => Ok(Rank::Class(offset)),
        'O' => Ok(Rank::Order(offset)),
        'F' => Ok(Rank::Family(offset)),
        'G' => Ok(Rank::Genus(offset)),
        'S' => Ok(Rank::Species(offset)),
        '-' => {
            // TODO: there has to be a better way to do that, maybe without the mutex business
            (*LAST_TAXONOMY_RANK_PARSED.lock().unwrap()).map_or_else(
                || {
                    Err(TaxRankParsingError::TaxRankParsingCannotInferRank(
                        String::from(string),
                    ))
                },
                |x| -> Result<Rank, TaxRankParsingError> { Ok(x.plus_one()) },
            )
        }
        _ => Err(TaxRankParsingError::InvalidRankCode(
            String::from(string),
            letter,
        )),
    }?;

    let mut old_tax_rank = LAST_TAXONOMY_RANK_PARSED.lock().unwrap();
    *old_tax_rank = Some(tax_rank);

    Ok(tax_rank)
}

impl<'de> Deserialize<'de> for Rank {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        parse_taxonomy_level(&string).map_err(serde::de::Error::custom)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        // Default implementation just delegates to `deserialize` impl.
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_order_taxonomy() {
        assert!(Rank::Domain(0) > Rank::Root(1));
        assert!(Rank::Domain(1) > Rank::Domain(0))
    }

    #[test_case("U", Rank::Unclassified(0); "ok_U")]
    #[test_case("U1", Rank::Unclassified(1); "ok_U1")]
    #[test_case("R", Rank::Root(0); "ok_R")]
    #[test_case("R1", Rank::Root(1); "ok_R1")]
    #[test_case("P", Rank::Phylum(0); "ok_P")]
    #[test_case("P1", Rank::Phylum(1); "ok_P1")]
    #[test_case("C", Rank::Class(0); "ok_C")]
    #[test_case("C1", Rank::Class(1); "ok_C1")]
    #[test_case("O", Rank::Order(0); "ok_O")]
    #[test_case("O1", Rank::Order(1); "ok_O1")]
    #[test_case("F", Rank::Family(0); "ok_F")]
    #[test_case("F1", Rank::Family(1); "ok_F1")]
    #[test_case("S", Rank::Species(0); "ok_S")]
    #[test_case("S1", Rank::Species(1); "ok_S1")]
    fn test_parse_tax_level(input: &str, expected: Rank) {
        pretty_assertions::assert_eq!(parse_taxonomy_level(input).unwrap(), expected);
    }

    #[test]
    fn test_plus_one() {
        pretty_assertions::assert_eq!(Rank::Kingdom(2).plus_one(), Rank::Kingdom(3))
    }

    #[test]
    #[should_panic]
    fn test_parse_tax_level_error_too_long() {
        // TODO: implements Eq on errors (fix csv and io errors first)
        parse_taxonomy_level("R11111").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_tax_level_error_invalid_code() {
        // TODO: implements Eq on errors (fix csv and io errors first)
        parse_taxonomy_level("L4").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_tax_level_error_offsetnotanumber() {
        // TODO: implements Eq on errors (fix csv and io errors first)
        parse_taxonomy_level("RR").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_tax_level_error_cannotinferprevious() {
        // reset
        {
            let mut old_tax_rank = LAST_TAXONOMY_RANK_PARSED.lock().unwrap();
            *old_tax_rank = None;
        }
        // TODO: implements Eq on errors (fix csv and io errors first)
        parse_taxonomy_level("-").unwrap();
    }
}
