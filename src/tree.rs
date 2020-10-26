use crate::parse_ident_organism_name;
use crate::ReportRecord;
use color_eyre::Report;
use core::convert::TryFrom;
use std::fmt::Display;

use daggy::{Dag, NodeIndex, Walker};
use tracing::instrument;

use crate::kraken::Organism;

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

impl IndentOrganism {
    pub fn inferior_indent(&self, than: &Self) -> bool {
        self.indent < than.indent
    }
}

impl TryFrom<ReportRecord> for IndentOrganism {
    type Error = crate::ErrorKind;

    #[instrument]
    fn try_from(value: ReportRecord) -> Result<Self, Self::Error> {
        let (_, (indent, name)) = parse_ident_organism_name(value.5.as_bytes()).unwrap(); // TODO: make error here

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

#[derive(Debug, Default)]
pub struct Tree {
    pub tree: Dag<IndentOrganism, u32, u32>,
    pub origin: NodeIndex,
    pub last_node_added_id: NodeIndex,
}

impl Tree {
    pub fn new(origin: IndentOrganism) -> Self {
        let mut tree: Dag<IndentOrganism, u32, u32> = Dag::new();
        let origin = tree.add_node(origin);
        let last_node_added_id = origin;

        Self {
            tree,
            origin,
            last_node_added_id,
        }
    }

    pub fn child(&mut self, parent: NodeIndex, node: IndentOrganism) -> &mut Self {
        let edge = 1;
        let (_, new_node_id) = self.tree.add_child(parent, edge, node);
        self.last_node_added_id = new_node_id;
        self
    }

    // find a parent with a lower indent value or default to the origin
    pub fn find_valid_parent_for(&self, organism: &IndentOrganism) -> NodeIndex {
        let mut parent_id = self.origin; // default value
        let mut parent_recursion = self
            .tree
            .recursive_walk(self.last_node_added_id, |g, n| g.parents(n).iter(g).last());

        while let Some((_, id)) = parent_recursion.walk_next(&self.tree) {
            let node = self.tree.node_weight(id).unwrap();
            if node.inferior_indent(organism) {
                parent_id = id;
                break;
            }
        }

        parent_id
    }
}

pub trait TaxonomyTreeReader<T>: Sized {
    fn read(_: T) -> Result<Self, Report>;
}
