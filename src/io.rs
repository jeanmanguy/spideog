use atty::Stream;
use color_eyre::{Help, Report};
use csv::Reader;
use dialoguer::Confirm;
use std::process;
use std::{fs::File, fs::OpenOptions, io, path::PathBuf};
use tracing::instrument;

use crate::{
    cli::args::{MultipleReports, SingleReport},
    BinError,
};

pub mod abundance_csv;
pub mod newick;
pub mod report;

/* ---------------------------------- Input --------------------------------- */

custom_derive! {
    #[derive(clap::Clap, Debug, PartialEq)]
    #[derive(EnumFromStr, EnumDisplay)]
    pub enum InputReportFormat {
        Kraken,
    }
}

#[instrument]
pub fn get_reader(input: &PathBuf, headers: bool) -> Result<Reader<File>, csv::Error> {
    csv::ReaderBuilder::new()
        .has_headers(headers)
        .delimiter(b'\t')
        .double_quote(false)
        .flexible(true)
        .from_path(input)
}

impl SingleReport {
    #[instrument]
    pub fn open_report(&self) -> Result<File, BinError> {
        let path = &self.path;
        open_file(path)
    }
}

impl MultipleReports {
    fn join_errors(errors: Vec<Result<File, BinError>>) -> Result<(), Report> {
        if errors.is_empty() {
            return Ok(());
        }

        errors
            .into_iter()
            .filter_map(|result| {
                if let Err(error) = result {
                    Some(error)
                } else {
                    None
                }
            })
            .fold(Err(eyre!("encountered multiple errors")), |report, e| {
                report.error(e)
            })
    }

    #[instrument]
    pub fn open_reports(&self) -> Result<Vec<File>, Report> {
        let readers: Vec<Result<File, BinError>> =
            self.paths.iter().map(|p| open_file(p)).collect();

        let (ok, errors) = readers.into_iter().partition(Result::is_ok);

        Self::join_errors(errors)?;

        Ok(ok.into_iter().map(Result::unwrap).collect::<Vec<File>>())
    }
}

#[instrument]
pub fn open_file(path: &PathBuf) -> Result<File, BinError> {
    let path = path;
    OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|err| BinError::Io {
            err,
            path: path.clone(),
        })
}

/* --------------------------------- OUTPUT --------------------------------- */

custom_derive! {
    #[derive(clap::Clap, Debug, PartialEq)]
    #[derive(EnumFromStr, EnumDisplay)]
    pub enum OutputPhyloFormat {
        Newick,
    }
}

custom_derive! {
    #[derive(clap::Clap, Debug, PartialEq)]
    #[derive(EnumFromStr, EnumDisplay)]
    pub enum OutputAbundanceFormat {
        Csv,
    }
}

#[derive(Debug, Clone)]
pub enum OutputKind {
    File(PathBuf),
    Stdout,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub kind: OutputKind,
    pub overwrite: bool,
}

impl From<Option<PathBuf>> for OutputKind {
    fn from(path: Option<PathBuf>) -> Self {
        path.map_or(Self::Stdout, |p| {
            if p == PathBuf::from(r"-") {
                Self::Stdout
            } else {
                Self::File(p)
            }
        })
    }
}

impl From<crate::cli::args::OutputFile> for Output {
    fn from(clap_output: crate::cli::args::OutputFile) -> Self {
        Self {
            kind: OutputKind::from(clap_output.path),
            overwrite: clap_output.overwrite,
        }
    }
}

impl Output {
    pub fn try_writtable(&self) -> Result<(), Report> {
        #[instrument]
        fn internal_can_open_file(path: &PathBuf, overwrite: bool) -> Result<(), Report> {
            if path.exists() {
                if overwrite {
                } else if atty::is(Stream::Stdout) {
                    if Confirm::new()
                        .with_prompt(format!("Overwrite `{}`?", path.display()))
                        .interact()?
                    {
                    } else {
                        process::exit(exitcode::NOPERM);
                    }
                } else {
                    {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::AlreadyExists,
                            "File already exists",
                        ))
                    }?
                }
            }
            Ok(())
        }

        match &self.kind {
            OutputKind::File(path) => internal_can_open_file(path, self.overwrite),
            OutputKind::Stdout => Ok(()),
        }
    }

    pub fn writer(&self) -> Result<Box<dyn io::Write>, Report> {
        match &self.kind {
            OutputKind::File(path) => Ok(Box::new(
                OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path)?,
            ) as Box<dyn io::Write>),
            OutputKind::Stdout => Ok(Box::new(io::stdout()) as Box<dyn io::Write>),
        }
    }
}
