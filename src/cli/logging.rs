use color_eyre::{eyre::Context, Report};
use log::LevelFilter;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode, WriteLogger};
use std::fs::OpenOptions;
use std::path::PathBuf;
use tracing::instrument;

#[derive(Clap, Debug)]
pub struct Logging {
    /// Log file (stdout if not present)
    #[clap(long, short, parse(from_os_str), global = true)]
    pub log: Option<PathBuf>,
    /// Show addditional information.
    #[clap(long, global = true)]
    pub verbose: bool,
}

impl Logging {
    #[instrument]
    pub fn setup(&self) -> Result<(), Report> {
        let verbosity: LevelFilter = if self.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        };

        match &self.log {
            Some(filepath) => Self::setup_file_log(verbosity, filepath),
            None => Self::setup_term_log(verbosity),
        }
    }

    #[instrument]
    fn setup_file_log(verbosity: LevelFilter, filepath: &PathBuf) -> Result<(), Report> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(false)
            .create(true)
            .open(filepath)?;
        WriteLogger::init(
            verbosity,
            ConfigBuilder::new().set_time_format_str("%F %R%:z").build(),
            file,
        )
        .wrap_err_with(|| {
            format!(
                "Failed to setup the writer logger for file {}",
                filepath.display()
            )
        })?;

        Ok(())
    }

    #[instrument]
    fn setup_term_log(verbosity: LevelFilter) -> Result<(), Report> {
        TermLogger::init(
            verbosity,
            ConfigBuilder::new()
                .set_time_level(LevelFilter::Off)
                .build(),
            TerminalMode::Stderr,
        )
        .wrap_err("Failed to setup the writer logger for stdout")?;

        Ok(())
    }
}
