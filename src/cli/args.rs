use std::path::PathBuf;

use clap::{Clap, ValueHint};

// #[derive(Clap, Debug, Clone)]
// pub struct KrakenReport {
//     /// Kraken reports
//     #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(true))]
//     pub reports: Vec<PathBuf>,
// }

// #[derive(Clap, Debug, PartialEq)]
// pub enum ExtractKind {
//     #[clap(alias = "p")]
//     Phylo,
//     #[clap(alias = "d")]
//     Data,
// }

// #[derive(Clap, Debug)]
// pub struct ExtractKind2 {
//     /// Extract taxonomy tree
//     #[clap(long, conflicts_with("data"))]
//     phylo: bool,
//     /// extract data
//     #[clap(long, conflicts_with("phylo"))]
//     data: bool
// }

// #[derive(Clap, Debug)]
// pub struct Extract {
//     #[clap(arg_enum, name = "kind", case_insensitive(true))]
//     pub kind: ExtractKind
// }

#[derive(Clap, Debug)]
pub struct SingleReport {
    /// A single Kraken report
    #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(false), takes_value(true))]
    pub path: PathBuf,
    /// Input report format
    #[clap(long = "report-format", name = "report-format", arg_enum, case_insensitive(true), global(true), default_value("Kraken"))]
    pub format: crate::io::InputReportFormat,
    /// Does the kraken report has headers
    #[clap(long = "has-headers", takes_value(false))]
    pub headers: bool
}

#[derive(Clap, Debug)]
pub struct MultipleReports {
    /// Multiple Kraken reports
    #[clap(name = "FILES", parse(from_os_str), value_hint = ValueHint::AnyPath, required(true), multiple(true), takes_value(true))]
    pub paths: Vec<PathBuf>,
    /// Input reports format (all reports must have the format)
    #[clap(long = "report-format", name = "report-format", arg_enum, case_insensitive(true), global(true), default_value("Kraken"))]
    pub format: crate::io::InputReportFormat,
    /// Does the kraken reports have headers (all or none)
    #[clap(long = "have-headers", takes_value(false))]
    pub headers: bool
}

#[derive(Clap, Debug, Clone)]
pub struct OutputFile {
    /// Output file [default: stdout (-)]
    #[clap(
        name = "output", 
        global(true), 
        long = "output", 
        parse(from_os_str), 
        value_hint = ValueHint::AnyPath, 
        takes_value(true),
    )]
    pub path: Option<PathBuf>,
    /// force overwriting exiting output file
    #[clap(
        long, 
        requires("output"), 
        global(true),
    )]
    pub overwrite: bool,
}

#[derive(Clap, Debug)]
pub struct InputReport {
    /// Input report format
    #[clap(long = "report-format", name = "report-format", arg_enum, case_insensitive(true), global(true), default_value("Kraken"))]
    pub format: crate::io::InputReportFormat,
}

#[derive(Clap, Debug)]
pub struct OutputPhylo {
    #[clap(flatten)]
    pub file: OutputFile,
    /// Output tree format
    #[clap(long = "format", name = "output-format", arg_enum, case_insensitive(true), default_value("Newick"))]
    pub format: crate::io::OutputPhyloFormat,
}


#[derive(Clap, Debug)]
pub struct OutputAbundance {
    #[clap(flatten)]
    pub file: OutputFile,
    /// Output abundance format
    #[clap(long = "format", name = "output-format", arg_enum, case_insensitive(true), default_value("csv"))]
    pub format: crate::io::OutputAbundanceFormat,
}


