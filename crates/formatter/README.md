# `formatter`

The official formatter used by Rome.

## Internal installation

```toml
formatter = { version = "0.0.0", path = "../formatter" }
```

## Usage

The foundation of the formatter relies on two pillars:

- the usage of a single [*trait*](https://doc.rust-lang.org/reference/items/traits.html);
- the creation of an intermediate IR via a series of helpers

Import the `ToFormatElement` trait and implement it for the data structure you need.

```rust
use formatter::{ToFormatElement, FormatElement, format_elements, token}

struct Buzz {
 blast: String
}

impl ToFormatElement for Buzz {
 fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
 // implementation goes here
 format_elements![token("_"), blast.as_str(), token("_")]
 }
}

```

## Rules when formatting AST nodes

-
