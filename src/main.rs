#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate eyre;

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
use color_eyre::{eyre::Report, eyre::WrapErr};

use displaydoc::Display;
use io::{get_output_file_name, read_report_tree, write_tree};
use kraken::{KrakenReportRecord, Organism};
use log::info;
use parser::parse_ident_organism_name;
use std::convert::TryFrom;
use thiserror::Error;
use tree::IndentOrganism;

#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Expected root with no indentation, found indentation level: `{0}`
    NonZeroIndentRoot(usize),
    /// No suitable parent found for node `{0}` of indent `{1}` and rank `{2}`
    NoSuitableParent(String, usize, taxonomy::TaxonomyRank),
    /// No node added to the tree
    NoNodeAdded,
    /// Failed to parse line `{0}`
    LineParsingError(usize),
    ///Io error
    #[error(transparent)]
    Io(#[from] std::io::Error),
    ///CSV parser error
    #[error(transparent)]
    Csv(#[from] csv::Error),
}

impl TryFrom<KrakenReportRecord> for IndentOrganism {
    type Error = Report;

    fn try_from(value: KrakenReportRecord) -> Result<Self, Self::Error> {
        let (_, (indent, name)) = parse_ident_organism_name(value.5.as_bytes()).unwrap();

        let organism_tree = Organism {
            taxonomy_level: value.3,
            name: String::from_utf8_lossy(name).trim().to_string(),
            taxonomy_id: value.4,
        };

        let node = IndentOrganism {
            indent,
            organism: organism_tree,
        };

        Ok(node)
    }
}
fn main() -> Result<(), Report> {
    cli::error::setup_error_hook()?;

    let opts: Opts = Opts::parse();
    opts.logging.setup().wrap_err("Failed to setup logging.")?;

    match opts.command {
        Command::Tree(sub_opts) => {
            // TODO check all files exists, gather errors with eyre
            // https://github.com/yaahc/color-eyre/blob/master/examples/multiple_errors.rs
            for report in sub_opts.files.reports.iter() {
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
