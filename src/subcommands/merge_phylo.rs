use color_eyre::{Help, Report};
use libspideog::{errors::SpideogError, tree::Tree};
use tracing::instrument;

use crate::{
    cli::subcommands::{CombineTrees, Runner},
    io::{newick::write_newick, report::ParseKrakenReport, Output},
};

type VecResultTrees = Vec<Result<Tree, SpideogError>>;

impl Runner for CombineTrees {
    #[instrument]
    fn run(self) -> Result<(), Report> {
        let readers = self.input.open_reports()?;
        let output = Output::from(self.output.file.clone());
        output.try_writtable()?;

        let (ok_trees, errors_trees): (VecResultTrees, VecResultTrees) = readers
            .into_iter()
            .map(|r| -> Result<Tree, SpideogError> {
                let mut csv_reader = csv::ReaderBuilder::new()
                    .has_headers(self.input.headers)
                    .delimiter(b'\t')
                    .double_quote(false)
                    .flexible(true)
                    .from_reader(r);

                let tree: Tree = ParseKrakenReport::parse(&mut csv_reader)?;

                Ok(tree)
            })
            .partition(Result::is_ok);

        if !errors_trees.is_empty() {
            return errors_trees
                .into_iter()
                .filter_map(|result| {
                    if let Err(error) = result {
                        Some(error)
                    } else {
                        None
                    }
                })
                .fold(Err(eyre!("encountered multiple errors")), |report, e| {
                    report.error(e)
                });
        }

        let mut trees_iter = ok_trees.into_iter().map(Result::unwrap);

        let merged_tree = trees_iter.try_fold(Tree::new(), Tree::try_combine_with)?;

        let mut writer = output.writer()?;
        match self.output.format {
            crate::io::OutputPhyloFormat::Newick => write_newick(&mut writer, &merged_tree)?,
        }

        Ok(())
    }
}
