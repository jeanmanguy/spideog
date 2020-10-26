// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// #![allow(clippy::missing_const_for_fn)]
// #![allow(clippy::multiple_crate_versions)]

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
use crate::io::Output;
use cli::{subcommands::Command, Opts};

use color_eyre::{eyre::Report, Help};
use displaydoc::Display;
use eyre::Context;
use io::{get_reader, read_report_tree, report::ParseKrakenReport};
use kraken::ReportRecord;
use parser::parse_ident_organism_name;
use thiserror::Error;
use tracing::{debug, info, instrument};
use tree::Tree;

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
    // ///IO error
    // #[error(transparent)]
    // Io(#[from] std::io::Error),
    // ///CSV parser error
    // #[error(transparent)]
    // Csv(#[from] csv::Error)
}

#[instrument]
fn main() -> Result<(), Report> {
    cli::install_tracing();
    cli::setup_error_hook()?;

    let opts: Opts = Opts::parse();

    match opts.command {
        // Command::Tree(sub_opts) => {
        //     info!("subcommand `tree`");
        //     // TODO check all files exists, gather errors with eyre
        //     // https://github.com/yaahc/color-eyre/blob/master/examples/multiple_errors.rs
        //     for report in &sub_opts.files.reports {
        //         let tree = read_report_tree(report, sub_opts.headers)?;
        //         let output_path = get_output_file_name(report, &sub_opts.prefix);
        //         info!("will write output to `{}`", &output_path.display());
        //         write_tree(tree, &output_path, &sub_opts.format, sub_opts.overwrite)?;
        //     }
        // }
        // Command::Info(info) => {
        //     dbg!(info);
        // }
        // Command::Convert(convert) => {
        //     dbg!(convert);
        //     //     match convert.kind {
        //     //     // cli::args::ExtractKind::Phylo => {
        //     //     //     info!("convert phylo");
        //     //     //     dbg!(&convert);
        //     //     //     debug!("{:?}", &convert);
        //     //     //     let tree = read_report_tree(&convert.file.report, false)?;
        //     //     //     let mut writer = output.writer()?;
        //     //     //     // write_tree(tree, &output_path, &sub_opts.format, sub_opts.overwrite)?;
        //     //     // }
        //     //     // cli::args::ExtractKind::Data => {
        //     //     //     info!("convert data");
        //     //     //     dbg!(convert);
        //     //     // }
        //     // }
        // }
        // Command::Merge(merge) => {
        //     dbg!(merge);
        // }
        // Command::Track(track) => {
        //     dbg!(track);
        // }
        Command::ConvertPhylo(args) => {
            args.run().wrap_err("Failed to convert taxonomy tree")?;
            // dbg!(&args);
            // debug!("checking if input file is readable");
            // let input = args.input.path;
            // let mut reader = get_reader(&input, false)
            //     .wrap_err_with(|| format!("Failed to read file `{}`", &input.display()))?;

            // debug!("checking if output file is writtable");
            // let output = Output::from(args.output.file);
            // output.try_writtable()?;

            // debug!("reading tree");

            // let tree: Tree = ParseKrakenReport::parse(&mut reader)
            //     .wrap_err_with(|| format!("Failed to parse file `{}`", &input.display()))
            //     .suggestion(
            //         "Try using the `--has-headers` option if your Kraken report has headers",
            //     )?;

            // todo!()
        }
        _ => {
            dbg!("hello");
        }
    }

    Ok(())
}
