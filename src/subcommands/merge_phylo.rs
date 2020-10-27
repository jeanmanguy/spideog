use color_eyre::{Help, Report};
use eyre::Context;
use libspideog::tree::Tree;
use tracing::instrument;

use crate::{
    cli::subcommands::{MergePhylo, Runner},
    io::{report::ParseKrakenReport, Output},
};

impl Runner for MergePhylo {
    #[instrument]
    fn run(self) -> Result<(), Report> {
        let readers = self.input.open_reports()?;
        let output = Output::from(self.output.file);
        output.try_writtable()?;

        let mut trees: Vec<Tree> = Vec::new();

        for reader in readers {
            let mut csv_reader = csv::ReaderBuilder::new()
                .has_headers(self.input.headers)
                .delimiter(b'\t')
                .double_quote(false)
                .flexible(true)
                .from_reader(reader);

            let tree: Tree = ParseKrakenReport::parse(&mut csv_reader)
                .wrap_err("oh no")
                // .wrap_err_with(|| format!("failed to parse file `{}`", &input.display()))
                .suggestion(
                    "try using the `--has-headers` option if your Kraken report has headers",
                )?;

            trees.push(tree);
        }

        Ok(())
    }
}
