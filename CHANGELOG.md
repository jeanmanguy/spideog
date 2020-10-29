# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- command `combine-trees` 
  - read multiple Kraken reports
  - write one Newick taxonomy tree
- second example Kraken report to test combining trees

### Modified

- changed `tree` to `convert-tree`
  - read only one file
  - write only one file (default: stdout)
- dev: split codebase between libspideog (src/lib.rs) and spideog (src/main.rs)
- dev: other refactoring and improvements of the codebase

## [0.1.1] - 2020-10-24

### Added 

- dev: continous integration builds for linux, osx, and windows
- error: add spantrace
- documentation: example kraken report and output
- documentation: links to downloads

### Modified

- bugfix: quotes and round brackets were added to the list of characters to escape in taxon name
- refactor: started to refactor to facilitate unit testing

## [0.1.0] - 2020-10-19

### Added

- command `tree` to convert the taxonomy tree from Kraken reports to newick format


<!-- next-url -->
[Unreleased]: https://github.com/jeanmanguy/spideog/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/jeanmanguy/spideog/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jeanmanguy/spideog/releases/tag/v0.1.0