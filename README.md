# 🐦 spideog - Command line utility for Kraken2 reports. <!-- omit in toc -->

![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)
![lastest version](https://img.shields.io/github/v/release/jeanmanguy/spideog)

[![Build Status](https://travis-ci.com/jeanmanguy/spideog.svg?branch=main)](https://travis-ci.com/jeanmanguy/spideog)
[![Rust](https://github.com/jeanmanguy/spideog/workflows/Rust/badge.svg?branch=main)](https://github.com/jeanmanguy/spideog/actions?query=workflow%3ARust)

This is a work in progress. The commands may change between released versions, please read the [CHANGELOG](CHANGELOG).

- [Goals](#goals)
- [Installation](#installation)
- [Usage](#usage)
  - [`convert-phylo`](#convert-phylo)
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
spideog convert-phylo <REPORT_FILE>
spideog convert-abundance <REPORT_FILE>
spideog merge-phylo <REPORT_FILE>...
spideog merge-abundance <REPORT_FILE>...
```

Windows: you will need to add the `.exe` extension to the commands.

### `convert-phylo`

Convert the taxonomy trees of a kraken report to the newick format.

The following command will generate the files `sample.tree`.

```sh
spideog convert-phylo --help
spideog convert-phylo sample.kreport --output sample.tree
```

### Example files <!-- omit in toc -->

- input: [tests/sample_data/sample.kreport](tests/sample_data/sample.kreport).
- output: [tests/sample_data/sample.tree](tests/sample_data/sample.tree)

#### Options <!-- omit in toc -->

- `--has_headers` necessary if the input report has headers
- `--output` output file path
- `--overwrite` force overwriting if the output file already exist
- `--report-format` input format (default: Kraken) [Only Kraken reports are supported at the moment]
- `--format` output format (default: newick) [Only newick is supported at the moment]

## Contributing

Please submit a bug report or a feature request [on the Github issues page](https://github.com/jeanmanguy/spideog/issues/new/choose).

## License

`spideog` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for
details.

## Credits

Cover picture: [Robin CC BY Greg Clarke](https://www.flickr.com/photos/leppre/25468458218)
