use std::collections::HashMap;
use std::iter::FromIterator;

use crate::kraken::{Fragments, Taxon};

pub type AbundanceData = HashMap<Taxon, Fragments>;

pub type SampleName = String;

#[derive(Debug, Default, PartialEq)]
pub struct SampleAbundance {
    pub name: SampleName,
    pub dataset: AbundanceData,
}

impl SampleAbundance {
    #[must_use]
    pub fn taxons(&self) -> Vec<Taxon> {
        self.dataset.keys().cloned().collect()
    }
}

impl From<(SampleName, AbundanceData)> for SampleAbundance {
    fn from(values: (SampleName, AbundanceData)) -> Self {
        Self {
            name: values.0,
            dataset: values.1,
        }
    }
}

pub type SamplesAbundanceData = Vec<SampleAbundance>; // FIXME remove

#[derive(Debug, Default, PartialEq)]
pub struct Samples {
    pub data: Vec<SampleAbundance>,
    pub unique_taxons: Vec<Taxon>,
}

impl Samples {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            unique_taxons: Vec::new(),
        }
    }

    fn add(&mut self, elem: SampleAbundance) {
        let new_taxons = elem.taxons();

        for taxon in new_taxons {
            if !self.unique_taxons.contains(&taxon) {
                self.unique_taxons.push(taxon);
            }
        }

        self.data.push(elem);
    }

    pub fn add_missing_taxons(&mut self) -> &mut Self {
        for datum in &mut self.data {
            for taxon in &self.unique_taxons {
                datum
                    .dataset
                    .entry(taxon.clone())
                    .or_insert_with(Fragments::default);
            }
        }

        self
    }
}

impl FromIterator<(SampleName, AbundanceData)> for Samples {
    fn from_iter<T: IntoIterator<Item = (SampleName, AbundanceData)>>(iter: T) -> Self {
        let mut samples = Self::new();

        for i in iter {
            let sample = SampleAbundance::from(i);
            samples.add(sample);
        }

        samples
    }
}
