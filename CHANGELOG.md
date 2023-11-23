# Change Log
All notable changes to this project will be documented in this file

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added
### Changed
### Removed

## [0.3.0] - 2023-11-23
### Added
- Multiple flags at once, eg.: `cw file.txt --words --chars -L`

## [0.2.1] - 2023-11-23
### Fixed
- Prevent newline, words and bytes printing if path doesn't exist

## [0.2.0] - 2023-11-21
### Added
- Character count (different from bytes)
- Arg parse with `clap`
- Max line length count

### Fixed
- If a file ended without a newline, it wouldn't count the last word, as the next character isn't a whitespace

### Removed
- Unused functions `count::file_newlines`, `count::file_words` and `count::files_bytes`

## [0.1.2] - 2023-11-19
### Changed
- Performance improvement when grabbing all information (newlines, words and bytes) from a file

## [0.1.1] - 2023-11-19
### Fixed
- Prevent panic when passing in multiple directories and no files as arguments

## [0.1.0] - 2023-11-18
### Added
- Project upload

### [Unreleased](https://github.com/appositum/cw/compare/0.3.0...dev)
### [0.3.0](https://github.com/appositum/cw/releases/tag/0.3.0)
### [0.2.1](https://github.com/appositum/cw/releases/tag/0.2.1)
### [0.2.0](https://github.com/appositum/cw/releases/tag/0.2.0)
### [0.1.2](https://github.com/appositum/cw/releases/tag/0.1.2)
### [0.1.1](https://github.com/appositum/cw/releases/tag/0.1.1)
### [0.1.0](https://github.com/appositum/cw/releases/tag/0.1.0)
