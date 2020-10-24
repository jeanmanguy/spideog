pub mod args;
pub mod error;
pub mod logging;
pub mod subcommands;

use std::path::PathBuf;

use clap::AppSettings;

#[derive(Debug, Clap)]
#[clap(author, about, version)]
#[clap(global_setting = AppSettings::ColoredHelp)]
#[clap(global_setting = AppSettings::ColorAuto)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
#[clap(global_setting = AppSettings::VersionlessSubcommands)]
#[clap(global_setting = AppSettings::DontCollapseArgsInUsage)]
#[clap(global_setting = AppSettings::ArgRequiredElseHelp)]
#[clap(global_setting = AppSettings::DisableHelpSubcommand)]
#[clap(global_setting = AppSettings::HelpRequired)]
#[clap(global_setting = AppSettings::UnifiedHelpMessage)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: subcommands::Command,
    // #[clap(flatten)]
    // pub logging: logging::Logging,
}
