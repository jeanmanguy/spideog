use super::args::{MultipleReports, OutputAbundance, OutputPhylo, SingleReport};
#[derive(Clap, Debug)]
#[non_exhaustive]
pub enum Command {
    // Info(Info),
    ConvertTree(ConvertTree),
    // ConvertAbundance(ConvertAbundance),
    CombineTree(CombineTree),
    // MergeAbundance(MergeAbundance),
    // Track(Track),
}

/// Extract diverse information about multiple reports
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct Info {
    #[clap(flatten)]
    pub input: MultipleReports,
    #[clap(flatten)]
    pub output: OutputAbundance,
}

/// Track one or multiple species across multiple reports
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct Track {
    #[clap(flatten)]
    pub input: MultipleReports,
    #[clap(flatten)]
    pub output: OutputAbundance,
}

/// Convert one report to one taxonomy tree
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct ConvertTree {
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

/// Combine multiple reports to one taxonomy tree
#[derive(Clap, Debug)]
#[clap(after_help = super::AFTER_HELP)]
pub struct CombineTree {
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
    fn run(self) -> Result<(), color_eyre::eyre::Report>;
}
