use color_eyre::{Help, Report};
use csv::ReaderBuilder;
use eyre::Context;
use libspideog::data::abundance::AbundanceData;
use tracing::instrument;

use crate::{
    cli::subcommands::{ConvertAbundance, Runner},
    io::{abundance_csv::WriteAbundanceCsv, report::ParseKrakenReport, Output},
};

impl Runner for ConvertAbundance {
    #[instrument]
    fn run(self) -> Result<(), Report> {
        let input = &self.input.path;

        let reader = self.input.open_report()?;
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(self.input.headers)
            .delimiter(b'\t')
            .double_quote(false)
            .flexible(true)
            .from_reader(reader);

        let output = Output::from(self.output.file);
        output.try_writtable()?;

        let data: AbundanceData = AbundanceData::parse(&mut csv_reader)
            .wrap_err_with(|| format!("failed to parse file `{}`", &input.display()))
            .suggestion("try using the `--has-headers` option if your Kraken report has headers")?;

        let mut writer = output.writer()?;

        match self.output.format {
            crate::io::OutputAbundanceFormat::Csv => {
                data.write_csv(&mut writer)
                    .wrap_err("failed to write output to CSV")?;
            }
        }

        Ok(())
    }
}
