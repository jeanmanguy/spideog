use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use color_eyre::{Help, Report};
use eyre::Context;
use tracing::{debug, instrument};

use crate::{
    cli::subcommands::{ConvertPhylo, Runner},
    io::newick::write_newick,
    io::open_file,
    io::{get_reader, report::ParseKrakenReport, Output},
    tree::Tree,
};

impl Runner for ConvertPhylo {
    #[instrument]
    fn run(self) -> Result<(), Report> {
        let input = &self.input.path;

        let reader = open_file(input)
            .wrap_err_with(|| format!("cannot read file `{}`", &input.display()))?;

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
            crate::io::OutputPhyloFormat::Newick => write_newick(&mut writer, tree)?,
        }

        Ok(())
    }
}
