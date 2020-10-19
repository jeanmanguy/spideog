use daggy::{NodeIndex, Walker};
use std::io;

use crate::{tree::SpideogTree, utils::clean_name, ErrorKind};

pub fn write_newick<W>(writer: &mut W, tree: SpideogTree, root: NodeIndex) -> Result<(), ErrorKind>
where
    W: std::io::Write,
{
    write_children_recursively(writer, &tree, root, 0)?;
    write_end(writer)?;

    Ok(())
}

#[inline]
pub fn write_name_distance<W>(
    writer: &mut W,
    name: String,
    distance: usize,
) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(writer, "{}:{}", clean_name(name), distance)
}

#[inline]
pub fn write_end<W>(writer: &mut W) -> Result<(), io::Error>
where
    W: io::Write,
{
    writeln!(writer, ";")
}

pub fn write_children_recursively<W>(
    writer: &mut W,
    tree: &SpideogTree,
    node: NodeIndex,
    parent_indent: usize,
) -> Result<(), io::Error>
where
    W: io::Write,
{
    let mut child_walker = tree.children(node);
    let mut children = Vec::new();
    while let Some((_, node)) = child_walker.walk_next(tree) {
        children.push(node);
    }

    let node_data = tree.node_weight(node).unwrap();

    if children.is_empty() {
        write_name_distance(
            writer,
            node_data.organism.name.clone(),
            node_data.indent - parent_indent,
        )?;
    } else {
        writer.write_all(b"(")?;

        let mut children_iter = children.iter().peekable();

        while let Some(node_id) = children_iter.next() {
            write_children_recursively(writer, tree, *node_id, node_data.indent)?;

            // not the last child, add a comma
            if children_iter.peek().is_some() {
                writer.write_all(b",")?;
            }
        }

        writer.write_all(b")")?;

        write_name_distance(
            writer,
            node_data.organism.name.clone(),
            node_data.indent - parent_indent,
        )?;
    }

    Ok(())
}
