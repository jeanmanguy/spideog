# 🐦 spideog - Command line utility for Kraken2 reports.

![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)
![lastest version](https://img.shields.io/github/v/release/jeanmanguy/spideog)

[![Build Status](https://travis-ci.com/jeanmanguy/spideog.svg?branch=main)](https://travis-ci.com/jeanmanguy/spideog)
[![Rust](https://github.com/jeanmanguy/spideog/workflows/Rust/badge.svg?branch=main)](https://github.com/jeanmanguy/spideog/actions?query=workflow%3ARust)

This is a work in progress. The commands may change between released versions, please read the [CHANGELOG](CHANGELOG).

## Goals

The first goal of this project is to convert Kraken reports into standard file formats that can be easily read with R to allow people to craft thier own data visualisations and compute statistics more easily using the tidyverse, vegan, ape, and ggtree/treeio. The second goal is to get summary information from the Kraken reports directly from the command line.

Supports Kraken reports from [Kraken2](https://github.com/DerrickWood/kraken2) or from [Bracken](https://github.com/jenniferlu717/Bracken). 



## Installation

Binaries for Linux, OSX, and Windows are available in the [Github release page](https://github.com/jeanmanguy/spideog/releases). No dependencies are required.


## Usage

```sh
spideog --help
```

### `tree`

Convert the taxonomy trees of kraken reports to newick format.

The following command will generate the files `sample_1.tree` and `sample_2.tree`.

```sh
spideog tree sample_1.kreport sample_2.kreport
```

#### Example files

- examplar kraken report: [tests/sample_data/sample.kreport](tests/sample_data/sample.kreport).
- output: [tests/sample_data/sample.tree](tests/sample_data/sample.tree)


#### Options 

- `--has_headers` necessary if the reports has headers
- `--overwrite` force overwriting if the output file already exist
- `--prefix` prepend the prefix to the name of the output file


## Contributing

Please submit a bug report or a feature request [on the Github issues page](https://github.com/jeanmanguy/spideog/issues/new/choose).

## License

`spideog` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for
details.

## Cover picture

Credit: [Robin CC BY Greg Clarke](https://www.flickr.com/photos/leppre/25468458218)
