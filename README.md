# 🐦 spideog

Command line utility for Kraken2 reports.

## Installation

TBD

## Usage

```sh
spideog --help
```

### Example files

TBD

### `tree`

Convert the taxonomy trees of kraken reports to newick format.

The following command will generate the files `sample_1.tree` and `sample_2.tree`.

```sh
spideog tree sample_1.kreport sample_2.kreport
```

#### Options for the output files

- `--overwrite` 
- `--prefix` 

## License

`spideog` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for
details.