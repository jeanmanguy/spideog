use color_eyre::Report;
use csv::Writer;
use eyre::Context;
use libspideog::data::abundance::{AbundanceData, Samples};
use serde::Serialize;

pub trait WriteAbundanceCsv: Sized {
    fn write_csv<W: std::io::Write>(self, writer: &mut W) -> Result<(), Report> {
        let mut csv_writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .has_headers(true)
            .from_writer(writer);

        self.write_records(&mut csv_writer)?;

        Ok(())
    }

    fn write_records<W: std::io::Write>(self, csv_writer: &mut Writer<W>) -> Result<(), Report>;
}

#[derive(Serialize)]
struct RowAbundanceData {
    #[serde(rename = "taxon")]
    name: String,
    #[serde(rename = "taxid")]
    taxonomy_id: u64,
    #[serde(rename = "rank")]
    taxonomy_level: String,
    clade_percentage: f64,
    clade_count_reads: u64,
    taxon_count_reads: u64,
}

#[derive(Serialize)]
struct RowSampleAbundanceData {
    sample: String,
    #[serde(rename = "taxon")]
    name: String,
    #[serde(rename = "taxid")]
    taxonomy_id: u64,
    #[serde(rename = "rank")]
    taxonomy_level: String,
    clade_percentage: f64,
    clade_count_reads: u64,
    taxon_count_reads: u64,
}

impl WriteAbundanceCsv for AbundanceData {
    fn write_records<W: std::io::Write>(self, csv_writer: &mut Writer<W>) -> Result<(), Report> {
        for (taxon, abundance_data) in self {
            csv_writer
                .serialize(RowAbundanceData {
                    name: taxon.name.clone(),
                    taxonomy_id: taxon.taxonomy_id,
                    taxonomy_level: format!("{}", taxon.taxonomy_level),
                    clade_percentage: abundance_data.clade_percentage,
                    clade_count_reads: abundance_data.clade_count_reads,
                    taxon_count_reads: abundance_data.taxon_count_reads,
                })
                .wrap_err_with(|| format!("failed to write record for `{}`", taxon.name))?;
        }

        Ok(())
    }
}

impl WriteAbundanceCsv for Samples {
    fn write_records<W: std::io::Write>(self, csv_writer: &mut Writer<W>) -> Result<(), Report> {
        for sample in self.data {
            for (taxon, abundance_data) in &sample.dataset {
                csv_writer
                    .serialize(RowSampleAbundanceData {
                        sample: sample.name.clone(),
                        name: taxon.name.clone(),
                        taxonomy_id: taxon.taxonomy_id,
                        taxonomy_level: format!("{}", taxon.taxonomy_level),
                        clade_percentage: abundance_data.clade_percentage,
                        clade_count_reads: abundance_data.clade_count_reads,
                        taxon_count_reads: abundance_data.taxon_count_reads,
                    })
                    .wrap_err_with(|| {
                        format!(
                            "failed to write record for sample `{}` `{}`",
                            sample.name, taxon.name
                        )
                    })?;
            }
        }

        Ok(())
    }
}
