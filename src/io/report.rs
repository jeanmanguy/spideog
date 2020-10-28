use color_eyre::{eyre::Context, Help, Report};
use core::convert::TryFrom;
use csv::Reader;
use libspideog::{
    errors::SpideogError,
    kraken::ReportRecord,
    tree::{IndentOrganism, Tree},
};
use std::fs::File;
use tracing::instrument;

pub trait ParseKrakenReport: Sized {
    fn parse(reader: &mut Reader<File>) -> Result<Self, SpideogError>;
}

fn parse_origin_tree(
    first_line: Option<Result<ReportRecord, csv::Error>>,
) -> Result<Tree, SpideogError> {
    let first_line = first_line.ok_or(SpideogError::EmptyFile)?;
    let first_record: ReportRecord = first_line.map_err(SpideogError::KrakenParser)?;
    let origin = IndentOrganism::try_from(first_record)?;
    let mut taxonomy_tree = Tree::new();
    taxonomy_tree.with_origin(origin);
    Ok(taxonomy_tree)
}

impl ParseKrakenReport for Tree {
    #[instrument]
    fn parse(reader: &mut Reader<File>) -> Result<Self, SpideogError> {
        let first_line = reader.deserialize().next();

        let mut taxonomy_tree = parse_origin_tree(first_line)?;

        for result in reader.deserialize() {
            let record: ReportRecord = result.map_err(SpideogError::KrakenParser)?;

            let node = IndentOrganism::try_from(record)?;
            let parent = taxonomy_tree.find_valid_parent_for(&node);
            taxonomy_tree.child(parent, node);
        }

        Ok(taxonomy_tree)
    }
}
