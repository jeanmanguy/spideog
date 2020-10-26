use color_eyre::{eyre::Context, Help, Report};
use core::convert::TryFrom;
use csv::Reader;
use std::fs::File;
use tracing::instrument;

use crate::{kraken::ReportRecord, tree::IndentOrganism, tree::Tree};

pub trait ParseKrakenReport: Sized {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report>;
}

fn parse_origin_tree(first_line: Option<Result<ReportRecord, csv::Error>>) -> Result<Tree, Report> {
    let first_line = first_line.ok_or(crate::ErrorKind::EmptyFile)?;
    let first_record: ReportRecord = first_line.map_err(crate::ErrorKind::CsvParser)?;
    let origin = IndentOrganism::try_from(first_record)?;
    let taxonomy_tree = Tree::new(origin);
    Ok(taxonomy_tree)
}

impl ParseKrakenReport for Tree {
    #[instrument]
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report> {
        let first_line = reader.deserialize().next();

        let mut taxonomy_tree = parse_origin_tree(first_line)
            .wrap_err("failed to parse first line")
            .suggestion("make sure that the file is a Kraken2 report")?;

        for result in reader.deserialize() {
            let record: ReportRecord = result
                .wrap_err("failed to parse line")
                .suggestion("make sure that the file is a Kraken2 report")?;

            let node = IndentOrganism::try_from(record)?;
            let parent = taxonomy_tree.find_valid_parent_for(&node);
            taxonomy_tree.child(parent, node);
        }

        Ok(taxonomy_tree)
    }
}
