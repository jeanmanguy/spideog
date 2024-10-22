// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// #![allow(clippy::missing_const_for_fn)]
// #![allow(clippy::multiple_crate_versions)]
// #![allow(clippy::missing_errors_doc)]
// #![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

mod cli;
mod io;
mod subcommands;

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
    /// IO error with `{path}`
    Io {
        #[source]
        err: std::io::Error,
        path: std::path::PathBuf,
    },
    /// encountered multiple errors
    MultipleErrors,
}

#[instrument]
fn main() -> Result<(), Report> {
    cli::install_tracing();
    cli::setup_error_hook()?;

    let opts: Opts = Opts::parse();

    match opts.command {
        Command::ConvertTree(args) => {
            args.run().wrap_err("failed to convert taxonomy tree")?;
        }
        Command::CombineTrees(args) => {
            args.run().wrap_err("failed to combine taxonomy trees")?;
        }
        Command::ConvertAbundance(args) => {
            args.run()
                .wrap_err("failed to convert taxonomy abundance data")?;
        }
        Command::CombineAbundances(args) => {
            args.run().wrap_err("failed to combine abundance data")?;
        }
    }

    Ok(())
}
