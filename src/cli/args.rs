use clap::{Clap, ValueHint};
use std::path::PathBuf;

#[derive(Clap, Debug, Clone)]
pub struct KrakenReport {
    /// Kraken reports
    #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(true))]
    pub reports: Vec<PathBuf>,
}
