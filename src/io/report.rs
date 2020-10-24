use color_eyre::{eyre::Context, Help, Report};
use core::convert::TryFrom;
use csv::Reader;
use std::fs::File;

use crate::{kraken::ReportRecord, tree::IndentOrganism, tree::Tree};

pub trait ParseKrakenReport: Sized {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report>;
}

impl ParseKrakenReport for Tree {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report> {
        let first_record: ReportRecord = reader.deserialize().next().unwrap()?;
        let origin = IndentOrganism::try_from(first_record)?;
        let mut taxonomy_tree = Self::new(origin);

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
