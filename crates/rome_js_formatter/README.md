# `rome_js_formatter`

The official formatter used by Rome.

## Internal installation

```toml
rome_js_formatter = { version = "0.0.0", path = "../rome_js_formatter" }
```

## Usage

The foundation of the formatter relies on two pillars:

- the usage of the [*trait*](https://doc.rust-lang.org/reference/items/traits.html) generic `Format` trait and `FormatNode` for nodes.
- the creation of an intermediate IR via a series of helpers

Import the `FormatNode` trait and implement it for your Node.

```rust
use rome_js_formatter::{Format, FormatNode, FormatElement, format_elements, token};

struct Buzz {
 blast: String
}

impl FormatNode for Buzz {
 fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
 	// implementation goes here
 	format_elements![token("_"), blast.as_str(), token("_")]
 }
}

```

## Rules when formatting AST nodes

1. if a token is mandatory and the AST has that information, please use that token instead, for example:

	```rust
	fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let l_paren_yes = &self.l_paren_token()?.format(); // yes
		let l_paren_no = toke("("); // no
	}
	```

 1. for tokens that are not mandatory, use our helpers
 1. do not attempt to "fix" the code. If you know a token/node is mandatory, return `None` instead
