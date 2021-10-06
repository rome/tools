# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),

## [Unreleased]

### Added

- Added lexer benchmarks
- Added support for numeric separators

### Fixed

- Fixed `/\u{0}/ª\u{80}` yielding ranges inside char boundaries

## [0.1.3] - 2020-10-20

### Changed

- Switched from codespan-reporting to a custom errors crate

### Removed

- Removed `Lexer::strip_shebang` in favor of proper shebang handling

### Added

- Added handling of shebang

### Fixed

- Fixed handling of `/=` and `>>=`
- Fixed handling of `\u{xxxx}` escapes in identifiers

## [0.1.2] - 2020-10-5

### Fixed

- Fixed proper handling of identifiers starting with unicode letters
- Fixed the lexer to use ID_Start and ID_Continue instead of XID_Start and XID_Continue

## [0.1.1] - 2020-10-3

### Fixed

- Fixed incorrect lexing of single line comments with unicode characters inside of them
