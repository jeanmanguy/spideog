use clap::{Clap, ValueHint};
use std::path::PathBuf;

// #[derive(Clap, Debug, Clone)]
// pub struct KrakenReport {
//     /// Kraken reports
//     #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(true))]
//     pub reports: Vec<PathBuf>,
// }

#[derive(Clap, Debug, Clone)]
pub struct SingleReport {
    /// a single Kraken report
    #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(false), takes_value(true), last(true))]
    pub report: PathBuf,
}

#[derive(Clap, Debug, Clone)]
pub struct MultipleReports {
    /// multiple Kraken reports
    #[clap(name = "FILES", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(true), takes_value(true), last(true))]
    pub reports: Vec<PathBuf>,
}

#[derive(Clap, Debug, Clone)]
pub struct Output {
    /// output path
    #[clap(name = "output", long = "output", short = 'o', parse(from_os_str), value_hint = ValueHint::AnyPath, takes_value(true))]
    pub path: Option<PathBuf>,
    /// force overwriting exiting file
    #[clap(long, requires("output"))]
    pub overwrite: bool,
}

pub enum OutputKind {
    File(PathBuf),
    Stdout,
}
