use crate::parse_ident_organism_name;
use core::convert::TryFrom;
use crate::KrakenReportRecord;
use color_eyre::Report;
use std::fmt::Display;

use daggy::{Dag, NodeIndex, Walker};
use tracing::instrument;

use crate::{kraken::Organism, taxonomy::TaxonomyRank, ErrorKind};

#[derive(Debug)]
pub struct IndentOrganism {
    pub indent: usize,
    pub organism: Organism,
}

impl Display for IndentOrganism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.organism)
    }
}

impl TryFrom<KrakenReportRecord> for IndentOrganism {
    type Error = Report;

    #[instrument]
    fn try_from(value: KrakenReportRecord) -> Result<Self, Self::Error> {
        let (_, (indent, name)) = parse_ident_organism_name(value.5.as_bytes()).unwrap();

        let organism_tree = Organism {
            taxonomy_level: value.3,
            name: String::from_utf8_lossy(name).trim().to_string(),
            taxonomy_id: value.4,
        };

        let node = IndentOrganism {
            indent,
            organism: organism_tree,
        };

        Ok(node)
    }
}

// TODO: make as struct and impl functions above for it, also store root with it, and other  info (n species, lower taxonomy rank, etc.)
pub type SpideogTree = Dag<IndentOrganism, u32, u32>;


pub fn ancestors(node: NodeIndex, dag: &SpideogTree) -> Vec<NodeIndex> {
    let mut ancestors_vec = Vec::new();
    let mut parent_recursion = dag.recursive_walk(node, |g, n| g.parents(n).iter(g).last());
    while let Some((_, id)) = parent_recursion.walk_next(&dag) {
        ancestors_vec.push(id);
    }
    ancestors_vec
}

pub fn find_correct_parent(
    node: &IndentOrganism,
    last_node_id: NodeIndex,
    root: NodeIndex,
    dag: &SpideogTree,
) -> Result<NodeIndex, ErrorKind> {
    let last_node = dag.node_weight(last_node_id).unwrap();

    if last_node.indent < node.indent {
        tracing::debug!(
            "Parent of `{}` is previously added node `{}`",
            node.organism, last_node.organism
        );
        Ok(last_node_id)
    } else {
        tracing::debug!(
            "Parent of `{}` is not the previously added node `{}`, searching for a suitable parent",
            node.organism, last_node.organism
        );

        let parents = ancestors(last_node_id, dag);
        let suitable_parent = parents.iter().find(|id| dag[**id].indent < node.indent);

        if let Some(parent_id) = suitable_parent {
            let parent = dag.node_weight(*parent_id).unwrap();
            tracing::debug!(
                "Found suitable parent for `{}` => `{}`",
                node.organism, parent.organism
            );
            Ok(*parent_id)
        } else if node.organism.taxonomy_level <= TaxonomyRank::Domain(9) {
            Ok(root)
        } else {
            Err(ErrorKind::NoSuitableParent(
                node.organism.name.clone(),
                node.indent,
                node.organism.taxonomy_level,
            ))
        }
    }
}

pub fn add_root_to_tree(
    node: IndentOrganism,
    dag: &mut SpideogTree,
) -> Result<NodeIndex, ErrorKind> {
    if node.indent != 0 {
        return Err(ErrorKind::NonZeroIndentRoot(node.indent));
    }

    let root_node_id = dag.add_node(node);
    Ok(root_node_id)
}

