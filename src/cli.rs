pub mod args;
pub mod logging;
pub mod subcommands;

use clap::AppSettings;
use color_eyre::{Report, Result};

static AFTER_HELP: &str = "Thank you for using Spideog. Please send any feedback, bug report or feature request to the project's github page: https://github.com/jeanmanguy/spideog";

#[derive(Debug, Clap)]
#[clap(author, about, version)]
#[clap(global_setting = AppSettings::ColoredHelp)]
#[clap(global_setting = AppSettings::ColorAuto)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
// #[clap(global_setting = AppSettings::VersionlessSubcommands)]
#[clap(global_setting = AppSettings::DontCollapseArgsInUsage)]
#[clap(global_setting = AppSettings::GlobalVersion)]
#[clap(global_setting = AppSettings::ArgRequiredElseHelp)]
#[clap(global_setting = AppSettings::HelpRequired)]
#[clap(global_setting = AppSettings::UnifiedHelpMessage)]
// #[clap(global_setting = AppSettings::SubcommandPrecedenceOverArg)]
#[clap(after_help = AFTER_HELP)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: subcommands::Command,
    // #[clap(flatten)]
    // pub logging: logging::Logging,
}

pub fn setup_error_hook() -> Result<(), Report> {
    color_eyre::config::HookBuilder::default()
        .add_default_filters()
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
        .add_issue_metadata("version", crate_version!())
        .add_issue_metadata("architecture", std::env::consts::ARCH)
        .add_issue_metadata("OS", std::env::consts::OS)
        .issue_filter(|kind| match kind {
            color_eyre::ErrorKind::NonRecoverable(_) => true,
            color_eyre::ErrorKind::Recoverable(_) => false,
        })
        .install()
}

// Boilerplate: https://github.com/yaahc/color-eyre/blob/master/examples/usage.rs
// TODO: adjust for use
// TODO: move to logging.rs?
pub fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
