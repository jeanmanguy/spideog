use std::collections::HashMap;
use std::iter::FromIterator;

use crate::kraken::{Fragments, Organism};

pub type AbundanceData = HashMap<Organism, Fragments>;

pub type SampleName = String;

#[derive(Debug, Default, PartialEq)]
pub struct SampleAbundance {
    pub name: SampleName,
    pub dataset: AbundanceData,
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
    pub unique_taxons: Vec<Organism>,
}

impl Samples {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            unique_taxons: Vec::new(),
        }
    }

    fn add(&mut self, elem: SampleAbundance) {
        self.data.push(elem);

        // TODO: add missing taxon ids
    }

    pub fn add_missing_taxons(&mut self) -> &mut Self {
        todo!()
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
