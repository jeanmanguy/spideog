use atty::Stream;
use color_eyre::{eyre::Context, Help, Report};
use csv::Reader;
use dialoguer::Confirm;
use std::process;
use std::{ffi::OsStr, fs::File, fs::OpenOptions, io, path::PathBuf};
use tracing::instrument;

use self::{newick::write_newick, report::ParseKrakenReport};
use crate::tree::Tree;

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

pub fn read_report_tree<P>(path: P, headers: bool) -> Result<Tree, Report>
where
    P: Into<PathBuf>,
{
    #[instrument]
    fn internal_read_report_tree(path: PathBuf, headers: bool) -> Result<Tree, Report> {
        let mut reader = get_reader(&path, headers)
            .wrap_err_with(|| format!("Failed to read file `{}`", path.display()))?;
        ParseKrakenReport::parse(&mut reader)
            .wrap_err_with(|| format!("Failed to parse file `{}`", path.display()))
            .suggestion("Try using the `--has-headers` option if your Kraken report has headers")
    }
    internal_read_report_tree(path.into(), headers)
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

#[derive(Debug)]
pub enum OutputKind {
    File(PathBuf),
    Stdout,
}

#[derive(Debug)]
pub struct Output {
    kind: OutputKind,
    overwrite: bool,
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
                log::debug!("`{}` already exists", path.display());
                if overwrite {
                    log::debug!("Force overwriting `{}`", path.display());
                } else if atty::is(Stream::Stdout) {
                    if Confirm::new()
                        .with_prompt(format!("Overwrite `{}`?", path.display()))
                        .interact()?
                    {
                        log::debug!("overwriting file `{}`", path.display())
                    } else {
                        log::debug!("refused to overwrite file, exiting...");
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
            } else {
                log::debug!("Will create file `{}`", path.display());
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

// #[instrument]
// pub fn get_output_file_name(input: &PathBuf, prefix: &Option<String>) -> PathBuf {
//     let stem = input
//         .file_stem()
//         .unwrap_or_else(|| OsStr::new("kraken"))
//         .to_string_lossy();
//     let new_path = if let Some(prefix) = prefix {
//         format!("{}_{}.tree", prefix, stem)
//     } else {
//         format!("{}.tree", stem)
//     };
//     PathBuf::from(new_path)
// }

// pub fn write_tree(
//     tree: Tree,
//     output: &PathBuf,
//     format: &OutputPhyloFormat,
//     overwrite: bool,
// ) -> Result<(), Report> {
//     let mut writer: Box<dyn std::io::Write> =
//         get_writer(output, overwrite).wrap_err("Failed to write tree")?;

//     match format {
//         OutputPhyloFormat::Newick => {
//             write_newick(&mut writer, tree).wrap_err("Newick Serializer failed")?
//         }
//     }

//     Ok(())
// }

// pub fn can_open_file<P>(path: P, overwrite: bool) -> Result<(), Report>
// where
//     P: Into<PathBuf>,
// {
//     #[instrument]
//     fn internal_can_open_file(path: PathBuf, overwrite: bool) -> Result<(), Report> {
//         if path.exists() {
//             log::debug!("`{}` already exists", path.display());
//             if overwrite {
//                 log::debug!("Force overwriting `{}`", path.display());
//             } else if atty::is(Stream::Stdout) {
//                 if Confirm::new()
//                     .with_prompt(format!("Overwrite `{}`?", path.display()))
//                     .interact()?
//                 {
//                     log::debug!("overwriting file `{}`", path.display())
//                 } else {
//                     log::debug!("refused to overwrite file, exiting...");
//                     process::exit(exitcode::NOPERM);
//                 }
//             } else {
//                 {
//                     Err(std::io::Error::new(
//                         std::io::ErrorKind::AlreadyExists,
//                         "File already exists",
//                     ))
//                 }?
//             }
//         } else {
//             log::debug!("Will create file `{}`", path.display());
//         }
//         Ok(())
//     }
//     internal_can_open_file(path.into(), overwrite)
// }

// #[instrument]
// pub fn get_writer(output: &PathBuf, overwrite: bool) -> Result<Box<dyn io::Write>, Report> {
//     can_open_file(output, overwrite)
//         .wrap_err_with(|| format!("Impossible to write to file `{}`", output.display()))?;
//     Ok(Box::new(
//         OpenOptions::new()
//             .write(true)
//             .truncate(true)
//             .create(true)
//             .open(output)?,
//     ) as Box<dyn io::Write>)
// }
