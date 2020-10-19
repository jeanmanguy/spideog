use std::fmt::Display;

use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use std::sync::Mutex;

static LAST_TAXONOMY_RANK_PARSED: Lazy<Mutex<Option<TaxonomyRank>>> =
    Lazy::new(|| Mutex::new(None));

/// Taxonomy levels
///
///
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
            TaxonomyRank::Unclassified(i) => write!(f, "U{}", i),
            TaxonomyRank::Root(i) => write!(f, "R{}", i),
            TaxonomyRank::Domain(i) => write!(f, "D{}", i),
            TaxonomyRank::Kingdom(i) => write!(f, "K{}", i),
            TaxonomyRank::Phylum(i) => write!(f, "P{}", i),
            TaxonomyRank::Class(i) => write!(f, "C{}", i),
            TaxonomyRank::Order(i) => write!(f, "O{}", i),
            TaxonomyRank::Family(i) => write!(f, "F{}", i),
            TaxonomyRank::Genus(i) => write!(f, "G{}", i),
            TaxonomyRank::Species(i) => write!(f, "S{}", i),
        }
    }
}

pub fn parse_taxonomy_level<'de, D>(string: &str) -> Result<TaxonomyRank, D::Error>
where
    D: Deserializer<'de>,
{
    if string.len() > 2 {
        return Err(serde::de::Error::invalid_length(string.len(), &"1 or 2"));
    }

    let mut string_chars = string.chars();

    let letter = string_chars.next().unwrap();

    let mut offset = 0u32;
    if let Some(number) = string_chars.next() {
        if number.is_ascii_digit() {
            offset = number.to_digit(10u32).unwrap();
        } else {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char(number),
                &"a digit (0..9)",
            ));
        }
    }

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
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Char(letter),
                    &"Cannot infer the taxonomy rank of `-`, no previous rank given",
                ))
            }
        }
        _ => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Char(letter),
            &"R, D, K, P, C, O, F, G, S, U, or -",
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
        parse_taxonomy_level::<D>(&string)
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

    #[test]
    fn test_order_taxonomy() {
        assert!(TaxonomyRank::Domain(0) > TaxonomyRank::Root(1));
        assert!(TaxonomyRank::Domain(1) > TaxonomyRank::Domain(0))
    }
}
