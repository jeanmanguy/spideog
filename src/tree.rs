use std::fmt::Display;

use daggy::{Dag, NodeIndex, Walker};
use log::debug;

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

pub fn ancestors(node: NodeIndex, dag: &Dag<IndentOrganism, u32, u32>) -> Vec<NodeIndex> {
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
    dag: &Dag<IndentOrganism, u32, u32>,
) -> Result<NodeIndex, ErrorKind> {
    let last_node = dag.node_weight(last_node_id).unwrap();

    if last_node.indent < node.indent {
        debug!(
            "Parent of `{}` is previously added node `{}`",
            node.organism, last_node.organism
        );
        Ok(last_node_id)
    } else {
        debug!(
            "Parent of `{}` is not the previously added node `{}`, searching for a suitable parent",
            node.organism, last_node.organism
        );

        let parents = ancestors(last_node_id, dag);
        let suitable_parent = parents.iter().find(|id| dag[**id].indent < node.indent);

        if let Some(parent_id) = suitable_parent {
            let parent = dag.node_weight(*parent_id).unwrap();
            debug!(
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
    dag: &mut Dag<IndentOrganism, u32, u32>,
) -> Result<NodeIndex, ErrorKind> {
    if node.indent != 0 {
        return Err(ErrorKind::NonZeroIndentRoot(node.indent));
    }

    let root_node_id = dag.add_node(node);
    Ok(root_node_id)
}

// TODO: make as struct and impl functions above for it, also store root with it, and other  info (n species, lower taxonomy rank, etc.)
pub type SpideogTree = Dag<IndentOrganism, u32, u32>;
