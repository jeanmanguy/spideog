use crate::taxonomy::Rank;
use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum SpideogError {
    /// expected root with no indentation, found indentation level: `{0}`
    NonZeroIndentRoot(usize),
    /// no suitable parent found for node `{0}` of indent `{1}` and rank `{2}`
    NoSuitableParent(String, usize, Rank),
    /// no node added to the tree
    NoNodeAdded,
    /// failed to parse line `{0}`
    LineParsingError(usize),
    /// node not found
    NodeNotFound,
    /// edge not found
    EdgeNotFound,
    /// parse output error
    ParseOutputPathError,
    /// input file is empty
    EmptyFile,
    /// Kraken parser error
    KrakenParser(#[source] csv::Error),
}

#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum TaxRankParsingError {
    /// failed to parse taxonomy rank offset from `{0}`: `{1}` is not a number (0..9)
    OffsetNotANumber(String, char),
    /// failed to parse taxonomy rank from `{0}`: found length `{1}` expected 1 or 2
    InvalidLength(String, usize),
    /// failed to parse taxonomy rank from `{0}`: invalid rank code `{1}` expected R, D, K, P, C, O, F, G, S, U, or -
    InvalidRankCode(String, char),
    /// failed to parse taxonomy rank from `{0}`: cannot infer previous taxonomy rank from previous records
    TaxRankParsingCannotInferRank(String),
}
