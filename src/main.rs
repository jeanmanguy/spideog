#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::multiple_crate_versions)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

mod bracken;
mod cli;
mod io;
mod kraken;
mod parser;
mod subcommands;
mod taxonomy;
mod tree;
mod utils;

use crate::clap::Clap;
use cli::{
    subcommands::{Command, Runner},
    Opts,
};

use color_eyre::eyre::Report;
use displaydoc::Display;
use eyre::Context;
use kraken::ReportRecord;
use parser::parse_ident_organism_name;
use thiserror::Error;
use tracing::instrument;

#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// expected root with no indentation, found indentation level: `{0}`
    NonZeroIndentRoot(usize),
    /// no suitable parent found for node `{0}` of indent `{1}` and rank `{2}`
    NoSuitableParent(String, usize, taxonomy::Rank),
    /// no node added to the tree
    NoNodeAdded,
    /// failed to parse line `{0}`
    LineParsingError(usize),
    /// failed to parse taxonomy rank offset from `{0}`: `{1}` is not a number (0..9)
    TaxRankParsingOfffsetNotANumber(String, char), // TODO: find a better solution to this mess
    /// failed to parse taxonomy rank from `{0}`: found length `{1}` expected 1 or 2
    TaxRankParsingInvalidLength(String, usize),
    /// failed to parse taxonomy rank from `{0}`: invalid rank code `{1}` expected R, D, K, P, C, O, F, G, S, U, or -
    TaxRankParsingInvalidRankCode(String, char),
    /// failed to parse taxonomy rank from `{0}`: cannot infer previous taxonomy rank from previous records
    TaxRankParsingCannotInferRank(String),
    /// node not found
    NodeNotFound,
    /// parse output error
    ParseOutputPathError,
    /// input file is empty
    EmptyFile,
    /// IO error: `{0}`
    Io(std::io::Error),
    /// CSV parser error: `{0}`
    CsvParser(csv::Error),
}

#[instrument]
fn main() -> Result<(), Report> {
    cli::install_tracing();
    cli::setup_error_hook()?;

    let opts: Opts = Opts::parse();

    match opts.command {
        Command::ConvertPhylo(args) => {
            args.run().wrap_err("failed to convert taxonomy tree")?;
        }
        _ => todo!(),
    }

    Ok(())
}
