// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

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
mod taxonomy;
mod tree;
mod utils;

use crate::clap::Clap;
use cli::{subcommands::Command, Opts};

use color_eyre::eyre::Report;
use displaydoc::Display;
use io::{get_output_file_name, read_report_tree, write_tree};
use kraken::KrakenReportRecord;
use parser::parse_ident_organism_name;
use thiserror::Error;
use tracing::{info, instrument};

#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// expected root with no indentation, found indentation level: `{0}`
    NonZeroIndentRoot(usize),
    /// no suitable parent found for node `{0}` of indent `{1}` and rank `{2}`
    NoSuitableParent(String, usize, taxonomy::TaxonomyRank),
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
    // ///IO error
    // #[error(transparent)]
    // Io(#[from] std::io::Error),
    // ///CSV parser error
    // #[error(transparent)]
    // Csv(#[from] csv::Error),
}

// Boilerplate: https://github.com/yaahc/color-eyre/blob/master/examples/usage.rs
fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[instrument]
fn main() -> Result<(), Report> {
    install_tracing();
    cli::error::setup_error_hook()?;

    let opts: Opts = Opts::parse();
    // opts.logging.setup().wrap_err("Failed to setup logging.")?;

    match opts.command {
        Command::Tree(sub_opts) => {
            info!("subcommand `tree`");
            // TODO check all files exists, gather errors with eyre
            // https://github.com/yaahc/color-eyre/blob/master/examples/multiple_errors.rs
            for report in &sub_opts.files.reports {
                let (tree, root) = read_report_tree(report, sub_opts.headers)?;
                let output_path = get_output_file_name(report, &sub_opts.prefix);
                info!("will write output to `{}`", &output_path.display());
                write_tree(
                    tree,
                    root,
                    &output_path,
                    &sub_opts.format,
                    sub_opts.overwrite,
                )?;
            }
        }
    }

    Ok(())
}
