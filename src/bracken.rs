use crate::kraken::Organism;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct AbundanceValues {
    pub kraken_assigned_reads: u64,
    pub added_reads: u64,
    pub new_est_reads: u64,
    pub fraction_total_reads: f64,
}
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
pub struct BrackenRecord {
    #[serde(flatten)]
    pub organism: Organism,
    #[serde(flatten)]
    pub abundance_values: AbundanceValues,
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use indextree::Arena;
    // use std::collections::BTreeMap;

    // #[test]
    // fn bracken_works() {
    //     let mut rdr = csv::ReaderBuilder::new()
    //         .has_headers(true)
    //         .delimiter(b'\t')
    //         .from_path(r"C:\Users\Jean\Documents\spideog\_test_data\Sam9_species.bracken")
    //         .unwrap();
    //     // dbg!(rdr);

    //     let mut bracken = BTreeMap::new();

    //     for result in rdr.deserialize() {
    //         let record: BrackenRecord = result.unwrap();
    //         bracken.insert(record.organism, record.abundance_values);
    //     }

    //     println!("{:?}", bracken);
    // }
}
