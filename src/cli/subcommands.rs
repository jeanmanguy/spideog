use crate::io::OutputTreeFormat;

use super::args::KrakenReport;
#[derive(Clap, Debug)]
#[non_exhaustive]
pub enum Command {
    Tree(Tree),
}

/// Extract and convert the taxonomy tree from Kracken reports
///
/// Export trees to *.tree files using the name of the original files.
/// A prefix can be added using the `--prefix` option.
#[derive(Clap, Debug)]
pub struct Tree {
    /// Report don't have headers
    #[clap(long = "has-headers")]
    pub headers: bool,
    /// Output taxonomy format
    #[clap(
        name = "tree-format",
        long = "tree-format",
        short = 'f',
        default_value = "newick"
    )]
    #[clap(arg_enum, case_insensitive(true))]
    pub format: OutputTreeFormat,
    /// Prefix for the output files
    #[clap(name = "prefix", long = "prefix", short = 'p')]
    pub prefix: Option<String>,
    /// Flag to force overwriting of output files
    #[clap(long)]
    pub overwrite: bool,
    #[clap(flatten)]
    pub files: KrakenReport,
}
