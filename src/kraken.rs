use std::{fmt::Display, convert::TryFrom};

use tracing::instrument;

use crate::{errors::SpideogError, taxonomy::Rank, parser::parse_ident_organism_name};

pub type ReportRecord = (String, u64, u64, Rank, u64, String);
pub type Indent = usize;

#[derive(Clone, PartialEq, PartialOrd, Debug, Ord, Eq, Hash, Deserialize)]
pub struct Organism {
    #[serde(rename = "taxonomy_lvl")]
    pub taxonomy_level: Rank,
    pub name: String,
    pub taxonomy_id: u64,
}

impl Display for Organism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} taxid:{} rank:{}",
            self.name, self.taxonomy_id, self.taxonomy_level
        )
    }
}

impl TryFrom<ReportRecord> for Organism {
    type Error = SpideogError;

    #[instrument]
    fn try_from(value: ReportRecord) -> Result<Self, Self::Error> {
        let (_, (_, name)) = parse_ident_organism_name(value.5.as_bytes()).unwrap(); // TODO: make error here

        let organism = Organism {
            taxonomy_level: value.3,
            name: String::from_utf8_lossy(name).trim().to_string(),
            taxonomy_id: value.4,
        };
        
        Ok(organism)
    }
}


#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Fragments {
    pub clade_percentage: f64,
    pub clade_count_reads: u64,
    pub taxon_count_reads: u64,
}

impl TryFrom<ReportRecord> for Fragments {
    type Error = SpideogError;

    #[instrument]
    fn try_from(value: ReportRecord) -> Result<Self, Self::Error> {
        let percentage= value.0.parse::<f64>().map_err(|_e| SpideogError::Other)?;

        let fragments = Fragments {
            clade_percentage: percentage,
            clade_count_reads: value.1,
            taxon_count_reads: value.1,

        };

        Ok(fragments)
    }
}