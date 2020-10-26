use super::args::{MultipleReports, OutputAbundance, OutputPhylo, SingleReport};
#[derive(Clap, Debug)]
#[non_exhaustive]
pub enum Command {
    Info(Info),
    ConvertPhylo(ConvertPhylo),
    ConvertAbundance(ConvertAbundance),
    MergePhylo(MergePhylo),
    MergeAbundance(MergeAbundance),
    Track(Track),
}

/// Extract diverse information about multiple reports
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct Info {
    #[clap(flatten)]
    pub input: MultipleReports,
    #[clap(flatten)]
    pub output: OutputAbundance, // TODO: change
}

/// Track one or multiple species across multiple reports
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct Track {
    #[clap(flatten)]
    pub input: MultipleReports,
    #[clap(flatten)]
    pub output: OutputAbundance, // TODO: change
}

/// Convert one report to one taxonomy tree
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct ConvertPhylo {
    #[clap(flatten)]
    pub input: SingleReport,
    #[clap(flatten)]
    pub output: OutputPhylo,
}

/// Convert one report to one abundance table
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct ConvertAbundance {
    #[clap(flatten)]
    pub input: SingleReport,
    #[clap(flatten)]
    pub output: OutputAbundance,
}

/// Merge multiple reports to one taxonomy tree
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct MergePhylo {
    #[clap(flatten)]
    pub input: MultipleReports,
    #[clap(flatten)]
    pub output: OutputPhylo,
}

/// Merge multiple reports to one abundance table
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct MergeAbundance {
    #[clap(flatten)]
    pub input: MultipleReports,
    #[clap(flatten)]
    pub output: OutputAbundance,
}

pub trait Runner {
    fn run(self) -> Result<(), color_eyre::eyre::Report>; // Could later use args: &MainArgs
}
