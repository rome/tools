# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),

## [Unreleased]

## [0.3.0] - 2021-09-16

### Removed

- Removed `new_module`
- Removed `syntax::program::script` and `syntax::program::module` in favor of just `ast::program::parse`

### Changed

- Made `cur_event_pos` and `token_pos` private in favor of a specialized checkpoint struct
- Changed `rewind` to take a Checkpoint

### Added

#### 🌟 Added TypeScript support 🌟

- Added `util::contains_js_whitespace`
- Added parser benchmarks
- Added `syntax::typescript`
- Added the `stop_on_r_curly` parameter to `block_items`
- Added `Syntax`
- Added a new parameter to `Parser::new`

### Fixed

- Fixed infinite recursion with `switch {` and

```js
try{}
catch(){
finally{}
```

- Fixed `let_token` in VarDecl not working when there is trivia attached to it
- Fixed infinite recursion with `[[;]]`

### Changed

- Made "redundant strict mode declaration" have a primary label instead of a single secondary label
- Reduce `Event` memory usage by 833%
- Remove `Event::Error`
- Rename `TreeSink::error` to `TreeSink::errors` and make it take `Vec<ParserError>`
- Improved the throughput of the parser by 1050%+
- Improved the speed of the parser by 130%+
- Stopped using `parse_marker` in many places
- Switched the backing of `TokenSet` from `u128` to `[u128; 2]` to accomodate tokens with a value over 128

## [0.2.1] - 2020-10-20

### Changed

- Switched from codespan-reporting to a custom errors crate

### Added

- Added handling of shebang where the parser consumes shebang if exists at the start
- Added a Checkpoint struct to easily backtrack (rewind) the parser

### Fixed

- Fixed handling of `/=` and `>>=`
- Fixed `parse_marker` panicking on preceded markers which are later abandoned
- Fixed arrow expr parameters sometimes being `NAME_REF` instead of `NAME`

## [0.2.0] - 2020-10-8

### Changed

- Changed `CATCH_CLAUSE`'s error to be a Pattern according to the ECMAScript spec
- Changed `Parse::ok` to yield `Ok(T)` instead of `Err(T)` if there are only warnings

### Added

- Added an explicit error handler for `...foo = bar`

### Fixed

- Fixed infinite recursion with nested curly braces in object literals (`let a = {{}}`)
- Fixed infinite recursion with initializers in spread elements
- Fixed infinite recursion in array and object patterns with invalid tokens
- Fixed grouping expression not being allowed as assignment targets
- Fixed `arguments` being flagged as an invalid ident in non-strict mode

## [0.1.3] - 2020-10-5

### Fixed

- Fixed infinite recursion with block statement recovery
- Fixed infinite recursion with erroneous tokens as object property names (#30)
- Fixed infinite recursion with formal_parameters without a binding_element

## [0.1.2] - 2020-10-3

### Fixed

- Fixed automatic semicolon insertion for statements which have `}` after it
- Fixed keywords not being allowed in key value patterns
- Fixed LosslessTreeSink accessing unicode char boundaries

## [0.1.1] - 2020-10-3

### Fixed

- Fixed proper state handling in generators
- Fixed proper error reporting for function declarations without a name
- Fixed arrow functions with a newline after the arrow being parsed incorrectly
- Fixed potential_arrow_start state not becoming false after parsing an assign_expr was parsed
- Fixed class declaration parents being parsed as names and not lhs expressions
- Fixed identifier_name creating an empty NAME node if there was no identifier
