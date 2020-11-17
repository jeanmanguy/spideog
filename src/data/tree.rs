use color_eyre::Report;
use core::convert::TryFrom;
use std::fmt::Display;

use daggy::{Dag, NodeIndex, Walker};
use tracing::instrument;

use crate::{
    errors::SpideogError,
    kraken::{ReportRecord, Taxon},
    parser::parse_ident_organism_name,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IndentedTaxon {
    pub indent: usize,
    pub taxon: Taxon,
}

impl Display for IndentedTaxon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.taxon)
    }
}

impl IndentedTaxon {
    #[must_use]
    pub fn inferior_indent(&self, than: &Self) -> bool {
        self.indent < than.indent
    }
}

impl TryFrom<ReportRecord> for IndentedTaxon {
    type Error = SpideogError;

    #[instrument]
    fn try_from(value: ReportRecord) -> Result<Self, Self::Error> {
        let (_, (indent, name)) = parse_ident_organism_name(value.5.as_bytes()).unwrap(); // TODO: make error here

        let organism_tree = Taxon {
            taxonomy_level: value.3,
            name: String::from_utf8_lossy(name).trim().to_string(),
            taxonomy_id: value.4,
        };

        let node = Self {
            indent,
            taxon: organism_tree,
        };

        Ok(node)
    }
}

pub trait TaxonomyTreeReader<T>: Sized {
    fn read(_: T) -> Result<Self, Report>;
}

#[derive(Debug, Default)]
pub struct Tree {
    pub tree: Dag<IndentedTaxon, u32, u32>,
    pub origin: Option<NodeIndex>,
    pub last_node_added_id: Option<NodeIndex>,
}

impl Tree {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tree: Dag::new(),
            origin: None,
            last_node_added_id: None,
        }
    }

    pub fn with_origin(&mut self, origin: IndentedTaxon) -> &mut Self {
        let new_node_index = self.tree.add_node(origin);
        self.origin = Some(new_node_index);
        self.last_node_added_id = Some(new_node_index);

        self
    }

    pub fn child(&mut self, parent: NodeIndex, node: IndentedTaxon) -> &mut Self {
        let weight = 1;
        self.child_with_weight(parent, node, weight)
    }

    pub fn child_with_weight(
        &mut self,
        parent: NodeIndex,
        node: IndentedTaxon,
        weight: u32,
    ) -> &mut Self {
        let (_, new_node_id) = self.tree.add_child(parent, weight, node);
        self.last_node_added_id = Some(new_node_id);

        self
    }

    //  find a parent with a lower indent value or default to the origin
    pub fn find_valid_parent_for(&self, taxon: &IndentedTaxon) -> Result<NodeIndex, SpideogError> {
        // default value
        let mut parent_id = self
            .origin
            .ok_or_else(|| SpideogError::TreeNotInitialized)?;

        let previously_added_node = self
            .last_node_added_id
            .ok_or_else(|| SpideogError::TreeNotInitialized)?;

        if self
            .tree
            .node_weight(previously_added_node)
            .ok_or_else(|| SpideogError::NodeNotFound)?
            .inferior_indent(taxon)
        {
            // previously added node is a suitable parent for the next taxon
            return Ok(previously_added_node);
        }

        // we need to go up the tree to find an adequate parent
        let mut parent_recursion = self
            .tree
            .recursive_walk(previously_added_node, |g, n| g.parents(n).iter(g).last());

        while let Some((_, node_id)) = parent_recursion.walk_next(&self.tree) {
            let node = self
                .tree
                .node_weight(node_id)
                .ok_or_else(|| SpideogError::NodeNotFound)?;

            if node.inferior_indent(taxon) {
                parent_id = node_id;
                break;
            }
        }

        Ok(parent_id)
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
                    // FIXME: what todo?
                    // TODO:  make it an error with more info
                    // dbg!(&rhs_edge_source);
                    // dbg!(&rhs_edge_target);
                    // dbg!(self.tree.node_weight(self.origin.unwrap()));
                    panic!("source and target of an edge in RHS were not found in Self");
                }
                (None, Some(_)) => {
                    // panic
                    // possible diamond
                    panic!(
                        "source and edge node in RHS were not found in Self, but target was found"
                    );
                }
                (Some(parent), None) => {
                    self.child(parent, rhs_edge_target.clone());
                }
                (Some(s), Some(t)) => {
                    // increment weight of edge
                    // FIXME: some issues with different trees, can't found node that exist
                    let original_edge = self.tree.find_edge(s, t).ok_or_else(|| {
                        SpideogError::EdgeNotFound(
                            self.tree.node_weight(s).unwrap().clone(),
                            self.tree.node_weight(t).unwrap().clone(),
                        )
                    })?;

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
        let origin = IndentedTaxon {
            indent: 0,
            taxon: Taxon {
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
        let origin = IndentedTaxon {
            indent: 0,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let child = IndentedTaxon {
            indent: 0,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "child".to_string(),
                taxonomy_id: 1,
            },
        };

        let grand_child = IndentedTaxon {
            indent: 2,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "grand child".to_string(),
                taxonomy_id: 2,
            },
        };

        let mut tree = Tree::new();
        tree.with_origin(origin.clone());
        tree.child(NodeIndex::new(0), child.clone());
        tree.child(NodeIndex::new(1), grand_child);

        pretty_assertions::assert_eq!(tree.tree.edge_count(), 2);
        pretty_assertions::assert_eq!(tree.tree.node_count(), 3);
        pretty_assertions::assert_eq!(tree.tree.node_weight(NodeIndex::new(0)), Some(&origin));
        pretty_assertions::assert_eq!(tree.tree.node_weight(NodeIndex::new(1)), Some(&child));

        assert!(tree
            .tree
            .find_edge(NodeIndex::new(0), NodeIndex::new(1))
            .is_some());
        assert!(tree
            .tree
            .find_edge(NodeIndex::new(1), NodeIndex::new(2))
            .is_some());

        pretty_assertions::assert_eq!(
            tree.tree
                .parents(NodeIndex::new(2))
                .iter(&tree.tree)
                .next()
                .unwrap()
                .1,
            NodeIndex::new(1)
        )
    }

    #[test]
    fn test_find_valid_parent() {
        let origin = IndentedTaxon {
            indent: 0,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let child = IndentedTaxon {
            indent: 1,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "child".to_string(),
                taxonomy_id: 1,
            },
        };

        let grand_child = IndentedTaxon {
            indent: 2,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "grand child".to_string(),
                taxonomy_id: 2,
            },
        };

        let new_child = IndentedTaxon {
            indent: 2,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(3),
                name: "new_child".to_string(),
                taxonomy_id: 3,
            },
        };

        let new_child_child = IndentedTaxon {
            indent: 3,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(3),
                name: "new_child_child".to_string(),
                taxonomy_id: 4,
            },
        };

        let mut tree = Tree::new();
        tree.with_origin(origin);
        tree.child(NodeIndex::new(0), child);
        tree.child(NodeIndex::new(1), grand_child);

        let parent = tree.find_valid_parent_for(&new_child).unwrap();
        tree.child(parent, new_child);

        // pretty_assertions::assert_eq!(parent, NodeIndex::new(1));

        let parent = tree.find_valid_parent_for(&new_child_child).unwrap();
        tree.child(parent, new_child_child);

        pretty_assertions::assert_eq!(parent, NodeIndex::new(3));
    }

    #[test]
    fn test_try_combine_with() {
        let origin = IndentedTaxon {
            indent: 0,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(0),
                name: "root".to_string(),
                taxonomy_id: 0,
            },
        };

        let child = IndentedTaxon {
            indent: 1,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "child".to_string(),
                taxonomy_id: 1,
            },
        };

        let second_child = IndentedTaxon {
            indent: 1,
            taxon: Taxon {
                taxonomy_level: crate::taxonomy::Rank::Root(1),
                name: "second child".to_string(),
                taxonomy_id: 2,
            },
        };

        let grand_child = IndentedTaxon {
            indent: 2,
            taxon: Taxon {
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
        expected_tree.with_origin(origin);
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
