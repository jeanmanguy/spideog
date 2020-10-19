use atty::Stream;
use color_eyre::{eyre::Context, Help, Report};
use csv::Reader;
use daggy::NodeIndex;
use dialoguer::Confirm;
use log::{debug, warn};
use std::{ffi::OsStr, fs::File, fs::OpenOptions, io, path::PathBuf};

use self::newick::write_newick;
use crate::tree::SpideogTree;

pub mod newick;
pub mod report;

/* ---------------------------------- Input --------------------------------- */

custom_derive! {
    #[derive(clap::Clap, Debug, PartialEq)]
    #[derive(EnumFromStr, EnumDisplay)]
    pub enum InputReportFormat {
        KrakenTree,
    }
}

pub fn read_report_tree<P>(path: P, headers: bool) -> Result<(SpideogTree, NodeIndex), Report>
where
    P: Into<PathBuf>,
{
    let path = path.into();
    let mut reader = get_reader(&path, headers)
        .wrap_err_with(|| format!("Failed to read file `{}`", path.display()))?;
    report::read_kraken_report_tree(&mut reader)
        .wrap_err_with(|| format!("Failed to parse file `{}`", path.display()))
        .suggestion("Try using the `--has-headers` option if your Kraken report has headers")
}

/* --------------------------------- OUTPUT --------------------------------- */

custom_derive! {
    #[derive(clap::Clap, Debug, PartialEq)]
    #[derive(EnumFromStr, EnumDisplay)]
    pub enum OutputTreeFormat {
        Newick,
    }
}

pub fn get_output_file_name(input: &PathBuf, prefix: &Option<String>) -> PathBuf {
    let stem = input
        .file_stem()
        .unwrap_or_else(|| &OsStr::new("kraken"))
        .to_string_lossy();
    let new_path = if let Some(prefix) = prefix {
        format!("{}_{}.tree", prefix, stem)
    } else {
        format!("{}.tree", stem)
    };
    PathBuf::from(new_path)
}

pub fn write_tree(
    tree: SpideogTree,
    root: NodeIndex,
    output: &PathBuf,
    format: &OutputTreeFormat,
    overwrite: bool,
) -> Result<(), Report> {
    let mut writer: Box<dyn std::io::Write> =
        get_writer(output, overwrite).wrap_err("Failed to write tree")?;

    match format {
        OutputTreeFormat::Newick => {
            write_newick(&mut writer, tree, root).wrap_err("Newick Serializer failed")?
        }
    }

    Ok(())
}

pub fn can_open_file<P>(path: P, overwrite: bool) -> Result<(), Report>
where
    P: Into<PathBuf>,
{
    let path = path.into();
    if path.exists() {
        debug!("File `{}` already exists", path.display());
        if overwrite {
            warn!("Force overwriting `{}`", path.display());
        } else if atty::is(Stream::Stdout)
            && Confirm::new()
                .with_prompt(format!("Overwrite `{}`?", path.display()))
                .interact()?
        {
            debug!("overwriting file")
        } else {
            {
                Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "File already exists",
                ))
            }?
        }
    } else {
        debug!("Will create file `{}`", path.display());
    }
    Ok(())
}

pub fn get_writer(output: &PathBuf, overwrite: bool) -> Result<Box<dyn io::Write>, Report> {
    can_open_file(output, overwrite)
        .wrap_err_with(|| format!("Impossible to write to file `{}`", output.display()))?;
    Ok(Box::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(output)?,
    ) as Box<dyn io::Write>)
}

pub fn get_reader(input: &PathBuf, headers: bool) -> Result<Reader<File>, csv::Error> {
    csv::ReaderBuilder::new()
        .has_headers(headers)
        .delimiter(b'\t')
        .double_quote(false)
        .flexible(true)
        .from_path(input)
}
