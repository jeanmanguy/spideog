use color_eyre::{eyre::Context, Help, Report};
use core::convert::TryFrom;
use csv::Reader;
use std::fs::File;

use crate::{kraken::KrakenReportRecord, tree::IndentOrganism, tree::TaxonomyTree};

pub trait ParseKrakenReport: Sized {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report>;
}

impl ParseKrakenReport for TaxonomyTree {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report> {
        let first_record: KrakenReportRecord = reader.deserialize().next().unwrap()?;
        let origin = IndentOrganism::try_from(first_record)?;
        let mut taxonomy_tree = TaxonomyTree::new(origin);

        for result in reader.deserialize() {
            let record: KrakenReportRecord = result
                .wrap_err("failed to parse line")
                .suggestion("make sure that the file is a Kraken2 report")?;

            let node = IndentOrganism::try_from(record)?;
            let parent = taxonomy_tree.find_valid_parent_for(&node);
            taxonomy_tree.child(parent, node);
        }

        Ok(taxonomy_tree)
    }
}
