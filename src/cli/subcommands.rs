use crate::io::OutputTreeFormat;

use super::args::{MultipleReports, Output, SingleReport};
#[derive(Clap, Debug)]
#[non_exhaustive]
pub enum Command {
    // Tree(Tree),
    Info(Info),
    Convert(Convert),
    Merge(Merge),
    Track(Track),
}

// /// (deprecated) Extract and convert the taxonomy tree from Kracken reports
// ///
// /// Export trees to *.tree files using the name of the original files.
// /// A prefix can be added using the `--prefix` option.
// #[derive(Clap, Debug)]
// pub struct Tree {
//     /// Report don't have headers
//     #[clap(long = "has-headers")]
//     pub headers: bool,
//     /// Output taxonomy format
//     #[clap(
//         name = "tree-format",
//         long = "tree-format",
//         short = 'f',
//         default_value = "newick"
//     )]
//     #[clap(arg_enum, case_insensitive(true))]
//     pub format: OutputTreeFormat,
//     /// Prefix for the output files
//     #[clap(name = "prefix", long = "prefix", short = 'p')]
//     pub prefix: Option<String>,
//     /// Flag to force overwriting of output files
//     #[clap(long)]
//     pub overwrite: bool,
//     #[clap(flatten)]
//     pub files: KrakenReport,
// }

/// Conversion from a report to a table format of tree format
#[derive(Clap, Debug)]
pub struct Convert {
    #[clap(subcommand)]
    kind: Option<ConvertKind>,
}

/// Extract diverse information about multiple reports
#[derive(Clap, Debug)]
pub struct Info {
    #[clap(flatten)]
    files: MultipleReports,
    #[clap(flatten)]
    output: Output,
}

/// Merge multiple reports to one table or tree
#[derive(Clap, Debug)]
pub struct Merge {
    #[clap(subcommand)]
    kind: Option<MergeKind>,
}

/// Track one or multiple species across multiple reports
#[derive(Clap, Debug)]
pub struct Track {
    #[clap(flatten)]
    files: MultipleReports,
}

#[derive(Clap, Debug)]
#[non_exhaustive]
pub enum ConvertKind {
    Phylo(ConvertPhylo),
    Data(ConvertData),
}

#[derive(Clap, Debug)]
#[non_exhaustive]
pub enum MergeKind {
    Phylo(MergePhylo),
    Data(MergeData),
}

#[derive(Clap, Debug)]
pub struct ConvertPhylo {
    #[clap(flatten)]
    file: SingleReport,
    #[clap(flatten)]
    output: Output,
}

#[derive(Clap, Debug)]
pub struct ConvertData {
    #[clap(flatten)]
    file: SingleReport,
    #[clap(flatten)]
    output: Output,
}

#[derive(Clap, Debug)]
pub struct MergePhylo {
    #[clap(flatten)]
    file: MultipleReports,
    #[clap(flatten)]
    output: Output,
}

#[derive(Clap, Debug)]
pub struct MergeData {
    #[clap(flatten)]
    file: MultipleReports,
    #[clap(flatten)]
    output: Output,
}
