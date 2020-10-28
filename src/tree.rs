use color_eyre::Report;
use core::convert::TryFrom;
use std::fmt::Display;

use daggy::{Dag, NodeIndex, Walker};
use tracing::{info, instrument};

use crate::{
    errors::SpideogError,
    kraken::{Organism, ReportRecord},
    parser::parse_ident_organism_name,
};

#[derive(Debug, Eq, PartialEq, Clone)]
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
    #[must_use]
    pub const fn inferior_indent(&self, than: &Self) -> bool {
        self.indent < than.indent
    }
}

impl TryFrom<ReportRecord> for IndentOrganism {
    type Error = SpideogError;

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

pub trait TaxonomyTreeReader<T>: Sized {
    fn read(_: T) -> Result<Self, Report>;
}

#[derive(Debug, Default)]
pub struct Tree {
    pub tree: Dag<IndentOrganism, u32, u32>,
    pub origin: Option<NodeIndex>,
    pub last_node_added_id: Option<NodeIndex>,
}

impl Tree {
    #[must_use]
    pub fn new() -> Self {
        let mut tree: Dag<IndentOrganism, u32, u32> = Dag::new();

        Self {
            tree,
            origin: None,
            last_node_added_id: None,
        }
    }

    pub fn with_origin(&mut self, origin: IndentOrganism) -> &mut Self {
        self.origin = Some(self.tree.add_node(origin));
        self.last_node_added_id = self.origin;

        self
    }

    pub fn child(&mut self, parent: NodeIndex, node: IndentOrganism) -> &mut Self {
        let weight = 1;
        self.child_with_weight(parent, node, weight)
    }

    #[inline]
    pub fn child_with_weight(
        &mut self,
        parent: NodeIndex,
        node: IndentOrganism,
        weight: u32,
    ) -> &mut Self {
        let (_, new_node_id) = self.tree.add_child(parent, weight, node);
        self.last_node_added_id = Some(new_node_id);
        self
    }

    /// find a parent with a lower indent value or default to the origin
    // TODO: make it a try function
    #[must_use]
    pub fn find_valid_parent_for(&self, organism: &IndentOrganism) -> NodeIndex {
        let mut parent_id = self.origin.unwrap(); // default value // TODO: add error
        let last_id = self.last_node_added_id.unwrap(); // TODO: add error
        let mut parent_recursion = self
            .tree
            .recursive_walk(last_id, |g, n| g.parents(n).iter(g).last());

        while let Some((_, id)) = parent_recursion.walk_next(&self.tree) {
            let node = self.tree.node_weight(id).unwrap();
            if node.inferior_indent(organism) {
                parent_id = id;
                break;
            }
        }

        parent_id
    }

    pub fn try_combine_with(mut self, rhs: Self) -> Result<Self, SpideogError> {
        if self.origin.is_none() {
            return Ok(rhs);
        }

        for rhs_edge in rhs.tree.raw_edges().iter() {
            let rhs_edge_source = rhs
                .tree
                .node_weight(rhs_edge.source())
                .ok_or_else(|| SpideogError::NodeNotFound)?;
            let rhs_edge_target = rhs
                .tree
                .node_weight(rhs_edge.target())
                .ok_or_else(|| SpideogError::NodeNotFound)?;

            let original_nodes = self.tree.raw_nodes();

            let source_in_self: Option<NodeIndex<u32>> =
                original_nodes.iter().enumerate().find_map(|(index, node)| {
                    if &node.weight == rhs_edge_source {
                        Some(NodeIndex::new(index))
                    } else {
                        None
                    }
                });

            let target_in_self: Option<NodeIndex<u32>> =
                original_nodes.iter().enumerate().find_map(|(index, node)| {
                    if &node.weight == rhs_edge_target {
                        Some(NodeIndex::new(index))
                    } else {
                        None
                    }
                });

            match (source_in_self, target_in_self) {
                (None, None) => {
                    // panic
                    // no common node (not even root)
                    // FIXME: whattodo?
                    // TODO:  make it an error with more info
                    panic!("source and target of an edge in RHS were not found in Self");
                }
                (None, Some(_)) => {
                    // panic
                    // possible diamond
                    panic!(
                        "source an edge node in RHS were not found in Self, but target was found"
                    );
                }
                (Some(parent), None) => {
                    self.child(parent, rhs_edge_target.clone());
                }
                (Some(s), Some(t)) => {
                    // increment weight of edge
                    let original_edge = self
                        .tree
                        .find_edge(s, t)
                        .ok_or_else(|| SpideogError::EdgeNotFound)?;
                    self.tree
                        .update_edge(
                            s,
                            t,
                            self.tree
                                .edge_weight(original_edge)
                                .unwrap_or(&1_u32)
                                .checked_add(1_u32)
                                .unwrap_or(1_u32),
                        )
                        .unwrap();
                }
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test_case::test_case;

    #[test]
    fn test_new() {
        let origin = IndentOrganism {
            indent: 0,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let mut tree = Tree::new();
        tree.with_origin(origin.clone());

        pretty_assertions::assert_eq!(tree.tree.edge_count(), 0);
        pretty_assertions::assert_eq!(tree.tree.node_count(), 1);
        pretty_assertions::assert_eq!(tree.tree.node_weight(NodeIndex::new(0)), Some(&origin));

        // tree.tree.
    }

    #[test]
    fn test_child() {
        let origin = IndentOrganism {
            indent: 0,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let child = IndentOrganism {
            indent: 0,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "child".to_string(),
                taxonomy_id: 1,
            },
        };

        let mut tree = Tree::new();
        tree.with_origin(origin.clone());
        tree.child(NodeIndex::new(0), child.clone());

        pretty_assertions::assert_eq!(tree.tree.edge_count(), 1);
        pretty_assertions::assert_eq!(tree.tree.node_count(), 2);
        pretty_assertions::assert_eq!(tree.tree.node_weight(NodeIndex::new(0)), Some(&origin));
        pretty_assertions::assert_eq!(tree.tree.node_weight(NodeIndex::new(1)), Some(&child));
    }

    #[test]
    fn test_find_valid_parent() {
        let origin = IndentOrganism {
            indent: 0,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let child = IndentOrganism {
            indent: 1,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "child".to_string(),
                taxonomy_id: 1,
            },
        };

        let grand_child = IndentOrganism {
            indent: 2,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "second child".to_string(),
                taxonomy_id: 2,
            },
        };

        let new_child = IndentOrganism {
            indent: 2,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(3),
                name: "new_child".to_string(),
                taxonomy_id: 3,
            },
        };

        let mut tree = Tree::new();
        tree.with_origin(origin);
        tree.child(NodeIndex::new(0), child);
        tree.child(NodeIndex::new(1), grand_child);

        let parent = tree.find_valid_parent_for(&new_child);

        pretty_assertions::assert_eq!(parent, NodeIndex::new(1));
    }

    #[test]
    fn test_try_combine_with() {
        let origin = IndentOrganism {
            indent: 0,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let child = IndentOrganism {
            indent: 1,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "child".to_string(),
                taxonomy_id: 1,
            },
        };

        let second_child = IndentOrganism {
            indent: 1,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "second child".to_string(),
                taxonomy_id: 2,
            },
        };

        let grand_child = IndentOrganism {
            indent: 2,
            organism: Organism {
                taxonomy_level: crate::taxonomy::Rank::Root(2),
                name: "grand child".to_string(),
                taxonomy_id: 3,
            },
        };

        let mut tree_1 = Tree::new();
        tree_1.with_origin(origin.clone());
        tree_1.child(NodeIndex::new(0), child.clone());
        tree_1.child(NodeIndex::new(0), second_child.clone());

        let mut tree_2 = Tree::new();
        tree_2.with_origin(origin.clone());
        tree_2.child(NodeIndex::new(0), child.clone());
        tree_2.child(NodeIndex::new(1), grand_child.clone());

        let mut expected_tree = Tree::new();
        expected_tree.with_origin(origin.clone());
        expected_tree.child_with_weight(NodeIndex::new(0), child, 2);
        expected_tree.child(NodeIndex::new(0), second_child);
        expected_tree.child(NodeIndex::new(1), grand_child);

        let combined_tree = tree_1.try_combine_with(tree_2).unwrap();

        pretty_assertions::assert_eq!(
            combined_tree.tree.edge_count(),
            expected_tree.tree.edge_count()
        );

        pretty_assertions::assert_eq!(
            combined_tree.tree.node_count(),
            expected_tree.tree.node_count()
        );

        pretty_assertions::assert_eq!(
            combined_tree
                .tree
                .raw_edges()
                .iter()
                .map(|e| e.weight)
                .collect::<Vec<u32>>(),
            expected_tree
                .tree
                .raw_edges()
                .iter()
                .map(|e| e.weight)
                .collect::<Vec<u32>>()
        );
    }
}
