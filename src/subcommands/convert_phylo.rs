use color_eyre::{Help, Report};
use eyre::Context;
use libspideog::tree::Tree;
use tracing::instrument;

use crate::{
    cli::subcommands::{ConvertTree, Runner},
    io::newick::write_newick,
    io::{report::ParseKrakenReport, Output},
};

impl Runner for ConvertTree {
    #[instrument]
    fn run(self) -> Result<(), Report> {
        let input = &self.input.path;

        let reader = self.input.open_report()?;

        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(self.input.headers)
            .delimiter(b'\t')
            .double_quote(false)
            .flexible(true)
            .from_reader(reader);

        let output = Output::from(self.output.file);
        output.try_writtable()?;

        let tree: Tree = ParseKrakenReport::parse(&mut csv_reader)
            .wrap_err_with(|| format!("failed to parse file `{}`", &input.display()))
            .suggestion("try using the `--has-headers` option if your Kraken report has headers")?;

        let mut writer = output.writer()?;

        match self.output.format {
            crate::io::OutputPhyloFormat::Newick => write_newick(&mut writer, &tree)?,
        }

        Ok(())
    }
}
