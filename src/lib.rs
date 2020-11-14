// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// #![allow(clippy::missing_const_for_fn)]
// #![allow(clippy::multiple_crate_versions)]
// #![allow(clippy::missing_errors_doc)]
// #![allow(clippy::module_name_repetitions)]

pub mod bracken;
pub mod data;
pub mod errors;
pub mod kraken;
pub mod parser;
pub mod taxonomy;

#[macro_use]
extern crate serde;
