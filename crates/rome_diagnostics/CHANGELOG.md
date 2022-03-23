# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),

## [Unreleased]

## [0.2.0] - 2021-09-16

### Fixed

- Fixed lsp diagnostic conversion not working correctly

### Added

- Added the `Formatter` trait for describing structs which can emit diagnostics in a certain way
- Added the `ShortFormatter` which emits diagnostics in an eslint-like style

### Changed

- Changed codespan backend to render notes with severity correctly
- Improved note rendering to account for severity length and indent correctly, as well as add a border
  between lines if the note spans multiple lines

## [0.1.1]

### Added

- Added multiple utility methods to `Span`
