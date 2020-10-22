use crate::parse_ident_organism_name;
use crate::KrakenReportRecord;
use color_eyre::Report;
use core::convert::TryFrom;
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

        let node = Self {
            indent,
            organism: organism_tree,
        };

        Ok(node)
    }
}

// TODO: make as struct and impl functions above for it, also store root with it, and other  info (n species, lower taxonomy rank, etc.)
// pub type SpideogTree = Dag<IndentOrganism, u32, u32>;

#[derive(Debug, Default)]
pub struct TaxonomyTree {
    pub tree: Dag<IndentOrganism, u32, u32>,
    pub root: Option<NodeIndex>,
    pub last_node_added_id: Option<NodeIndex>,
}

// impl Default for TaxonomyTree {
//     fn default() -> Self {
//         Sel
//     }
// }

impl TaxonomyTree {
    pub fn ancestors_of(&self, node: NodeIndex) -> Vec<NodeIndex> {
        let mut ancestors_vec = Vec::new();
        let mut parent_recursion = self
            .tree
            .recursive_walk(node, |g, n| g.parents(n).iter(g).last());
        while let Some((_, id)) = parent_recursion.walk_next(&self.tree) {
            ancestors_vec.push(id);
        }
        ancestors_vec
    }

    pub fn last_node_added(&self) -> Option<&IndentOrganism> {
        self.last_node_added_id
            .and_then(|i| self.tree.node_weight(i))
    }

    pub fn find_correct_parent_of(
        &self,
        organism_to_add: &IndentOrganism,
    ) -> Result<NodeIndex, ErrorKind> {
        let last_node = self.last_node_added();

        if let Some(last_node) = last_node {
            if last_node.indent < organism_to_add.indent {
                tracing::debug!(
                    "Parent of `{}` is previously added node `{}`",
                    organism_to_add.organism,
                    last_node.organism
                );
                Ok(self.last_node_added_id.unwrap())
            } else {
                tracing::debug!(
                    "Parent of `{}` is not the previously added node `{}`, searching for a suitable parent",
                    organism_to_add.organism,
                    last_node.organism
                );

                let parents = self.ancestors_of(self.last_node_added_id.unwrap()); //TODO: remove unwrap after removing option on TaxonomyTree
                let suitable_parent = parents
                    .iter()
                    .find(|id| self.tree[**id].indent < organism_to_add.indent);

                if let Some(parent_id) = suitable_parent {
                    let parent = self.tree.node_weight(*parent_id).unwrap();
                    tracing::debug!(
                        "Found suitable parent for `{}` => `{}`",
                        organism_to_add.organism,
                        parent.organism
                    );
                    Ok(*parent_id)
                } else if organism_to_add.organism.taxonomy_level <= TaxonomyRank::Domain(9) {
                    Ok(self.root.unwrap())
                } else {
                    Err(ErrorKind::NoSuitableParent(
                        organism_to_add.organism.name.clone(),
                        organism_to_add.indent,
                        organism_to_add.organism.taxonomy_level,
                    ))
                }
            }
        } else {
            panic!("empty tree")
        }
    }

    pub fn root(&mut self, node: IndentOrganism) -> Result<&mut Self, ErrorKind> {
        if node.indent != 0 {
            return Err(ErrorKind::NonZeroIndentRoot(node.indent));
        }

        self.root = Some(self.tree.add_node(node));
        Ok(self)
    }
}
