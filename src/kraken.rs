use std::fmt::Display;

use crate::taxonomy::TaxonomyRank;

pub type KrakenReportRecord = (String, u64, u64, TaxonomyRank, u64, String);

// pub struct KrakenReportRecord {
//     pub percentage: String,
//     pub fragments_clade: u64,
//     pub fragments_taxon: u64,
//     pub rank: TaxonomyRank,
//     pub taxid: u64,
//     pub indented_name: String,
// }

pub type KrakenIndent = usize;

#[derive(Clone, PartialEq, PartialOrd, Debug, Ord, Eq, Hash, Deserialize)]
pub struct Organism {
    #[serde(rename = "taxonomy_lvl")]
    pub taxonomy_level: TaxonomyRank,
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

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct KrakenFragments {
    pub percentage: f64,
    pub count_clade: u64,
    pub count_taxon: u64,
}
