use color_eyre::{Help, Report};
use eyre::Context;
use libspideog::{
    data::abundance::{AbundanceData, SampleName, Samples},
    errors::SpideogError,
};
use std::iter::FromIterator;
use tracing::instrument;

use crate::{
    cli::subcommands::{CombineAbundance, Runner},
    io::{abundance_csv::WriteAbundanceCsv, report::ParseKrakenReport, Output},
};

type VecResultAbundanceData = Vec<Result<(SampleName, AbundanceData), SpideogError>>;

impl Runner for CombineAbundance {
    #[instrument]
    fn run(self) -> Result<(), Report> {
        let sample_names: Vec<SampleName> = self
            .input
            .paths
            .iter()
            .map(|p| p.file_stem().unwrap().to_string_lossy().into())
            .collect();
        let readers = self.input.open_reports()?;
        let output = Output::from(self.output.file.clone());
        output.try_writtable()?;

        let (ok_abundance_data, errors_abundance_data): (
            VecResultAbundanceData,
            VecResultAbundanceData,
        ) = readers
            .into_iter()
            .zip(sample_names)
            .map(
                |(file, sample_name)| -> Result<(SampleName, AbundanceData), SpideogError> {
                    let mut csv_reader = csv::ReaderBuilder::new()
                        .has_headers(self.input.headers)
                        .delimiter(b'\t')
                        .double_quote(false)
                        .flexible(true)
                        .from_reader(file);

                    let tree: AbundanceData = ParseKrakenReport::parse(&mut csv_reader)?;

                    Ok((sample_name, tree))
                },
            )
            .partition(Result::is_ok);

        if !errors_abundance_data.is_empty() {
            return errors_abundance_data
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

        let mut samples = Samples::from_iter(ok_abundance_data.into_iter().map(Result::unwrap));

        if self.add_missing_taxons {
            samples.add_missing_taxons();
        }

        let mut writer = output.writer()?;
        match self.output.format {
            crate::io::OutputAbundanceFormat::Csv => {
                samples
                    .write_csv(&mut writer)
                    .wrap_err("failed to write output to CSV")?;
            }
        }

        Ok(())
    }
}
