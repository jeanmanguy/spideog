use core::convert::TryFrom;
use color_eyre::{Report, eyre::Context, Help};
use csv::Reader;
use daggy::NodeIndex;
use std::fs::File;

use crate::{
    kraken::KrakenReportRecord,
    tree::add_root_to_tree, tree::find_correct_parent, tree::IndentOrganism, tree::SpideogTree,
};



pub fn read_kraken_report_tree(
    reader: &mut Reader<File>,
) -> Result<(SpideogTree, NodeIndex), Report> {
    let mut tree = SpideogTree::new();

    let mut root: Option<NodeIndex> = None;
    let mut last_node_id: Option<NodeIndex> = None;
    let mut new_node_id: NodeIndex;

    for result in reader.deserialize() {
        let record: KrakenReportRecord = result
            .wrap_err("failed to parse line")
            .suggestion("make sure that the file is a Kraken2 report")?;

        let node = IndentOrganism::try_from(record)?;

        if root.is_none() {
            new_node_id = add_root_to_tree(node, &mut tree)?;
            root = Some(new_node_id);
            last_node_id = Some(new_node_id);
        } else if let Some(id) = last_node_id {
            let parent = find_correct_parent(&node, id, root.unwrap(), &tree)?;

            let (_, new_node_id) = tree.add_child(parent, 1, node);
            last_node_id = Some(new_node_id);
        } else {
            panic!("Tree didn't initialize properly");
        }
    }

    if let Some(root) = root {
        Ok((tree, root))
    } else {
        // TODO: find a better way to do that
        panic!("Failed to add nodes")
    }
}
