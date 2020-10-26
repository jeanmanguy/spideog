use std::fs::OpenOptions;

use color_eyre::{Help, Report};
use eyre::Context;
use tracing::{debug, instrument};

use crate::{
    cli::subcommands::ConvertPhylo,
    io::{get_reader, report::ParseKrakenReport, Output},
    tree::Tree,
};

impl ConvertPhylo {
    #[instrument]
    pub fn run(self) -> Result<(), Report> {
        let input = &self.input.path;

        let reader = OpenOptions::new()
            .read(true)
            .write(false)
            .open(input)
            .wrap_err("cannot read")?;

        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(self.input.headers)
            .delimiter(b'\t')
            .double_quote(false)
            .flexible(true)
            .from_reader(reader);

        let output = Output::from(self.output.file);
        output.try_writtable()?;

        let tree: Tree = ParseKrakenReport::parse(&mut csv_reader)
            .wrap_err_with(|| format!("Failed to parse file `{}`", &input.display()))
            .suggestion("Try using the `--has-headers` option if your Kraken report has headers")?;

        Ok(())
    }
}
