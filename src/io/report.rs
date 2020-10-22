use color_eyre::{eyre::Context, Help, Report};
use core::convert::TryFrom;
use csv::Reader;
use daggy::NodeIndex;
use std::fs::File;

use crate::{kraken::KrakenReportRecord, tree::IndentOrganism, tree::TaxonomyTree};

pub trait ParseKrakenReport: Sized {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report>;
}

impl ParseKrakenReport for TaxonomyTree {
    fn parse(reader: &mut Reader<File>) -> Result<Self, Report> {
        let mut taxonomy_tree = Self::default();
        let mut new_node_id: NodeIndex;

        for result in reader.deserialize() {
            let record: KrakenReportRecord = result
                .wrap_err("failed to parse line")
                .suggestion("make sure that the file is a Kraken2 report")?;

            let node = IndentOrganism::try_from(record)?;

            if taxonomy_tree.root.is_none() {
                taxonomy_tree.root(node)?;
                new_node_id = taxonomy_tree.root.unwrap();
                taxonomy_tree.last_node_added_id = Some(new_node_id);
            } else if taxonomy_tree.last_node_added_id.is_some() {
                let parent = taxonomy_tree.find_correct_parent_of(&node)?;

                let (_, new_node_id) = taxonomy_tree.tree.add_child(parent, 1, node); //TODO: move to taxonomy tree, only return node id
                taxonomy_tree.last_node_added_id = Some(new_node_id);
            } else {
                panic!("Tree didn't initialize properly");
            }
        }

        Ok(taxonomy_tree)
    }
}
