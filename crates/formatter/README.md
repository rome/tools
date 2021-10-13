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

1. if a token is mandatory and the AST has that information, please use that token instead, for example:

	```rust
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
	let l_paren_yes = formatter.format_token(&self.l_paren_token().unwrap()); // yes
	let l_paren_no = toke("("); // no
	}
	```

 1. for tokens that are not mandatory, use our helpers
 1. do not attempt to "fix" the code. If you know a token/node is mandatory, throw an error instead
