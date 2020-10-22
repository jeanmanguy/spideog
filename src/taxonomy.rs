use crate::ErrorKind;
use std::fmt::Display;

use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use std::sync::Mutex;
use tracing::instrument;

static LAST_TAXONOMY_RANK_PARSED: Lazy<Mutex<Option<TaxonomyRank>>> =
    Lazy::new(|| Mutex::new(None));

/// Taxonomy levels
///
/// the u32 offset represents sub-clade (e.g. parvorder, subfamily, etc.)
#[derive(Clone, PartialEq, Debug, PartialOrd, Ord, Eq, Hash, Copy)]
pub enum TaxonomyRank {
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
impl Display for TaxonomyRank {
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

#[instrument]
pub fn parse_taxonomy_level(string: &str) -> Result<TaxonomyRank, ErrorKind> {
    // TODO: add previous tax rank here, make it pure
    if string.len() > 2 {
        return Err(ErrorKind::TaxRankParsingInvalidLength(
            String::from(string),
            string.len(),
        ));
    }

    let mut string_chars = string.chars();

    let letter = string_chars.next().unwrap();

    let offset: u32 = if let Some(number) = string_chars.next() {
        if number.is_ascii_digit() {
            number.to_digit(10_u32).unwrap()
        } else {
            return Err(ErrorKind::TaxRankParsingOfffsetNotANumber(
                String::from(string),
                number,
            ));
        }
    } else {
        0_u32
    };

    let tax_rank = match letter {
        'U' => Ok(TaxonomyRank::Unclassified(offset)),
        'R' => Ok(TaxonomyRank::Root(offset)),
        'D' => Ok(TaxonomyRank::Domain(offset)),
        'K' => Ok(TaxonomyRank::Kingdom(offset)),
        'P' => Ok(TaxonomyRank::Phylum(offset)),
        'C' => Ok(TaxonomyRank::Class(offset)),
        'O' => Ok(TaxonomyRank::Order(offset)),
        'F' => Ok(TaxonomyRank::Family(offset)),
        'G' => Ok(TaxonomyRank::Genus(offset)),
        'S' => Ok(TaxonomyRank::Species(offset)),
        '-' => {
            if let Some(previous_tax_rank) = *LAST_TAXONOMY_RANK_PARSED.lock().unwrap() {
                match previous_tax_rank {
                    TaxonomyRank::Unclassified(i) => Ok(TaxonomyRank::Unclassified(i + 1)),
                    TaxonomyRank::Root(i) => Ok(TaxonomyRank::Root(i + 1)),
                    TaxonomyRank::Domain(i) => Ok(TaxonomyRank::Domain(i + 1)),
                    TaxonomyRank::Kingdom(i) => Ok(TaxonomyRank::Kingdom(i + 1)),
                    TaxonomyRank::Phylum(i) => Ok(TaxonomyRank::Phylum(i + 1)),
                    TaxonomyRank::Class(i) => Ok(TaxonomyRank::Class(i + 1)),
                    TaxonomyRank::Order(i) => Ok(TaxonomyRank::Order(i + 1)),
                    TaxonomyRank::Family(i) => Ok(TaxonomyRank::Family(i + 1)),
                    TaxonomyRank::Genus(i) => Ok(TaxonomyRank::Genus(i + 1)),
                    TaxonomyRank::Species(i) => Ok(TaxonomyRank::Species(i + 1)),
                }
            } else {
                Err(ErrorKind::TaxRankParsingCannotInferRank(String::from(
                    string,
                )))
            }
        }
        _ => Err(ErrorKind::TaxRankParsingInvalidRankCode(
            String::from(string),
            letter,
        )),
    }?;

    let mut old_tax_rank = LAST_TAXONOMY_RANK_PARSED.lock().unwrap();
    *old_tax_rank = Some(tax_rank);

    Ok(tax_rank)
}

impl<'de> Deserialize<'de> for TaxonomyRank {
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
        assert!(TaxonomyRank::Domain(0) > TaxonomyRank::Root(1));
        assert!(TaxonomyRank::Domain(1) > TaxonomyRank::Domain(0))
    }

    #[test_case("U", TaxonomyRank::Unclassified(0); "ok_U")]
    #[test_case("U1", TaxonomyRank::Unclassified(1); "ok_U1")]
    #[test_case("R", TaxonomyRank::Root(0); "ok_R")]
    #[test_case("R1", TaxonomyRank::Root(1); "ok_R1")]
    #[test_case("P", TaxonomyRank::Phylum(0); "ok_P")]
    #[test_case("P1", TaxonomyRank::Phylum(1); "ok_P1")]
    #[test_case("C", TaxonomyRank::Class(0); "ok_C")]
    #[test_case("C1", TaxonomyRank::Class(1); "ok_C1")]
    #[test_case("O", TaxonomyRank::Order(0); "ok_O")]
    #[test_case("O1", TaxonomyRank::Order(1); "ok_O1")]
    #[test_case("F", TaxonomyRank::Family(0); "ok_F")]
    #[test_case("F1", TaxonomyRank::Family(1); "ok_F1")]
    #[test_case("S", TaxonomyRank::Species(0); "ok_S")]
    #[test_case("S1", TaxonomyRank::Species(1); "ok_S1")]
    fn test_parse_tax_level(input: &str, expected: TaxonomyRank) {
        pretty_assertions::assert_eq!(parse_taxonomy_level(input).unwrap(), expected);
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
