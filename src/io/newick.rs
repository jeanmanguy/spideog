use color_eyre::Report;
use daggy::{NodeIndex, Walker};
use eyre::ContextCompat;
use libspideog::data::tree::Tree;
use std::{borrow::Cow, io};
use tracing::instrument;

pub fn write_newick<W>(writer: &mut W, tree: &Tree) -> Result<(), Report>
where
    W: std::io::Write,
{
    write_children_recursively(writer, tree, tree.origin.unwrap(), 0)?; // TODO: add error / panic Tree not initilialised
    write_end(writer)?;

    Ok(())
}

#[inline]
pub fn write_name_distance<W, S>(writer: &mut W, name: S, distance: usize) -> Result<(), io::Error>
where
    W: io::Write,
    S: AsRef<str>,
{
    write!(writer, "{}", format_name_distance(name, distance))
}

#[inline]
fn format_name_distance<S: AsRef<str>>(name: S, distance: usize) -> String {
    format!("{}:{}", clean_name(name.as_ref()), distance)
}

#[inline]
pub fn write_end<W>(writer: &mut W) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(writer, "{}", format_end())
}

#[instrument]
#[inline]
fn format_end() -> String {
    String::from(";\n")
}

fn is_trouble(c: char) -> bool {
    c == ' ' || c == '.' || c == ',' || c == '=' || c == '[' || c == ']' || c == '/' || c == ':'
}

// based from https://lise-henry.github.io/articles/optimising_strings.html
// not going to use regex just for that
pub fn clean_name<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();

    let first_trouble_character = input.find(is_trouble);
    if let Some(first_trouble_character) = first_trouble_character {
        let mut output = String::from(&input[0..first_trouble_character]);
        output.reserve(input.len() - first_trouble_character);
        let rest = input[first_trouble_character..].chars();
        for c in rest {
            match c {
                ' ' | '-' | '/' | ':' => output.push_str("_"),
                '.' | ',' | '[' | ']' | '(' | ')' | '\'' | '\"' => {}
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}

pub fn write_children_recursively<W>(
    writer: &mut W,
    tree: &Tree,
    node: NodeIndex,
    parent_indent: usize,
) -> Result<(), Report>
where
    W: io::Write,
{
    let mut child_walker = tree.tree.children(node);
    let mut children = Vec::new();
    while let Some((_, node)) = child_walker.walk_next(&tree.tree) {
        children.push(node);
    }

    let node_data = tree.tree.node_weight(node).wrap_err("node not found")?;
    let distance = node_data
        .indent
        .checked_sub(parent_indent)
        .wrap_err_with(|| {
            format!(
                "failed to compute new distance: node {} - parent {}",
                node_data.indent, parent_indent
            )
        })?;

    if children.is_empty() {
        write_name_distance(writer, &node_data.organism.name, distance)?;
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

        write_name_distance(writer, &node_data.organism.name, distance)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&("Homo sapiens", 2), "Homo_sapiens:2")]
    #[test_case(&("Bacteroidetes/Chlorobi group", 1), "Bacteroidetes_Chlorobi_group:1")]
    fn test_format_name_distance<S: AsRef<str>>(input: &(S, usize), expected: S) {
        assert_eq!(
            format_name_distance(input.0.as_ref(), input.1),
            expected.as_ref()
        );
    }
}
