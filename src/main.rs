#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

// mod bracken;
mod cli;
mod io;
// mod kraken;
// mod parser;
mod subcommands;
// mod taxonomy;
// mod tree;
mod utils;

use crate::clap::Clap;
use cli::{
    subcommands::{Command, Runner},
    Opts,
};

use color_eyre::eyre::Report;
use displaydoc::Display;
use eyre::Context;
use thiserror::Error;
use tracing::instrument;

#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum BinError {
    /// IO error: `{0}`
    Io(std::io::Error),
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
