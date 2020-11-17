# 🐦 spideog - Command line utility for Kraken2 reports. <!-- omit in toc -->

[![lastest version](https://img.shields.io/github/v/release/jeanmanguy/spideog)](https://github.com/jeanmanguy/spideog/releases/tag/v0.1.2-alpha.1)

[![Build Status](https://travis-ci.com/jeanmanguy/spideog.svg?branch=main)](https://travis-ci.com/jeanmanguy/spideog)
[![Rust](https://github.com/jeanmanguy/spideog/workflows/Rust/badge.svg?branch=main)](https://github.com/jeanmanguy/spideog/actions?query=workflow%3ARust)

This is a work in progress. The commands may change between released versions, please read the [CHANGELOG](CHANGELOG).

- [Goals](#goals)
- [Installation](#installation)
- [Usage](#usage)
  - [`convert-tree`](#convert-tree)
  - [`convert-abundance`](#convert-abundance)
  - [`combine-trees`](#combine-trees)
  - [`combine-abundances`](#combine-abundances)
- [Contributing](#contributing)
- [License](#license)
- [Credits](#credits)

## Goals

The first goal of this project is to convert Kraken reports into standard file formats that can be easily read with R to allow people to craft thier own data visualisations and compute statistics more easily using the tidyverse, vegan, ape, and ggtree/treeio. The second goal is to get summary information from the Kraken reports directly from the command line.

Supports Kraken reports from [Kraken2](https://github.com/DerrickWood/kraken2) or from [Bracken](https://github.com/jenniferlu717/Bracken). 

## Installation

Binaries for Linux, OSX, and Windows are available in the [Github release page](https://github.com/jeanmanguy/spideog/releases). No dependencies are required.

## Usage

```sh
spideog --help
spideog --version
spideog convert-tree <REPORT_FILE>
spideog convert-abundance <REPORT_FILE>
spideog combine-trees <REPORT_FILE>...
spideog combine-abundances <REPORT_FILE>...
```

Windows: you will need to add the `.exe` extension to the commands.

### `convert-tree`

Convert the taxonomy tree of a Kraken report to the Newick format.

The following command will generate the file `converted.tree`.

```sh
spideog convert-tree sample.kreport --output converted.tree
```

### Example files <!-- omit in toc -->

- input: [tests/sample_data/sample.kreport](tests/sample_data/sample.kreport).
- output: [tests/sample_data/converted.tree](tests/sample_data/converted.tree)

#### Options <!-- omit in toc -->

- `--has-headers` necessary if the input report has headers
- `--output` output file path
- `--overwrite` force overwriting if the output file already exist
- `--report-format` input format (default: Kraken) [Only Kraken reports are supported at the moment]
- `--format` output format (default: newick) [Only newick is supported at the moment]

### `convert-abundance`

Convert the abundance data of a Kraken report to the CSV format.


The following command will generate the file `converted.csv`.

```sh
spideog convert-abundance sample.kreport --output converted.csv
```


### Example files <!-- omit in toc -->

- input: [tests/sample_data/sample.kreport](tests/sample_data/sample.kreport).
- output: [tests/sample_data/converted.csv](tests/sample_data/converted.csv)

#### Options <!-- omit in toc -->

- `--has-headers` necessary if the input report has headers
- `--output` output file path
- `--overwrite` force overwriting if the output file already exist
- `--report-format` input format (default: Kraken) [Only Kraken reports are supported at the moment]
- `--format` output format (default: CSV) [Only CSV is supported at the moment]


### `combine-trees`

Combine and convert taxonomy trees from multiple Kraken report (e.g. from different samples of the same experiment) to the Newick format.

The following command will generate the file `combined.tree`.

```sh
spideog combine-trees sample.kreport sample_2.kreport --output combined.tree
```

### Example files <!-- omit in toc -->

- inputs: [tests/sample_data/sample.kreport](tests/sample_data/sample.kreport) and [tests/sample_data/sample_2.kreport](tests/sample_data/sample_2.kreport).
- output: [tests/sample_data/combined.tree](tests/sample_data/combined.tree)

#### Options <!-- omit in toc -->

- `--has-headers` necessary if the input reports have headers
- `--output` output file path
- `--overwrite` force overwriting if the output file already exist
- `--report-format` input format (default: Kraken) [Only Kraken reports are supported at the moment]
- `--format` output format (default: newick) [Only newick is supported at the moment]


### `combine-abundances`

Combine and convert abundance data from multiple Kraken report (e.g. from different samples of the same experiment) to the CSV format.

The following command will generate the file `combined.csv`.

```sh
spideog combine-abundances sample.kreport sample_2.kreport --add-missing-taxons --output combined.csv
```

### Example files <!-- omit in toc -->

- inputs: [tests/sample_data/sample.kreport](tests/sample_data/sample.kreport) and [tests/sample_data/sample_2.kreport](tests/sample_data/sample_2.kreport).
- output: [tests/sample_data/combined.csv](tests/sample_data/combined.csv)


#### Options <!-- omit in toc -->

- `--add-missing-taxons` add missig taxons in some reports but present in other with zero values
- `--has-headers` necessary if the input report has headers
- `--output` output file path
- `--overwrite` force overwriting if the output file already exist
- `--report-format` input format (default: Kraken) [Only Kraken reports are supported at the moment]
- `--format` output format (default: CSV) [Only CSV is supported at the moment]


## Contributing

The project is maintained by Jean Manguy. Please submit a bug report or a feature request [on the Github issues page](https://github.com/jeanmanguy/spideog/issues/new/choose).

## License

`spideog` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for
details.

## Credits

Cover picture: [Robin CC BY Greg Clarke](https://www.flickr.com/photos/leppre/25468458218)
