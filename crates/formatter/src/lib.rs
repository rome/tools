//! Rome's official formatter.
//!
//! The crate exposes some API and utilities to implement the formatting logic.
//!
//! The formatter relies on an [IR], which allows to format any kind of data structure.
//!
//! In order to implement the formatting logic, you need to implement the trait [FormatValue] for
//! the data structure you want to format.
//!
//! Let's say, for example that you have a small data structure that represents a key/value data:
//!
//! ```rust,no_test
//! struct KeyValue {
//!     key: String,
//!     value: String
//! }
//! ```
//!
//! Now, we do want to create this IR for the data structure:
//! ```rust
//! use rslint_parser::SyntaxKind;
//! use rome_formatter::{format_tokens, format_token, FormatToken, FormatValue, FormatOptions, FormatContext};
//!
//! struct KeyValue {
//!     key: String,
//!     value: String
//! }
//!
//! impl FormatValue for KeyValue {
//!     fn format(&self, context: &mut FormatContext) -> FormatToken {
//!         format_tokens![
//!             context.tokens.double_quoted_string(self.key.as_str()),
//!             FormatToken::Space,
//!             context.tokens.get(SyntaxKind::FAT_ARROW, "=>"),
//!             FormatToken::Space,
//!             context.tokens.double_quoted_string(self.value.as_str())
//!         ]
//!     }
//! }
//!
//! fn my_function() {
//!     let key_value = KeyValue { key: String::from("lorem"), value: String::from("ipsum") };
//!     let mut context = FormatContext::default();
//!     let token = key_value.format(&mut context);
//!     let options = FormatOptions::default();
//!     let result = format_token(&token, options);
//!     assert_eq!(result.root().text(), "lorem => ipsum");
//! }
//!
//! ```
//! [IR]: https://en.wikipedia.org/wiki/Intermediate_representation

mod format_json;
mod format_token;
mod format_tokens_macro;
mod intersperse;
mod printer;
mod token_cache;
mod tokens;

use crate::format_json::tokenize_json;
use std::{fs::File, io::Read, str::FromStr};

use crate::printer::{PrintResult, Printer};
pub use format_token::{
	FormatToken, GroupToken, IfBreakToken, IndentToken, LineMode, LineToken, ListToken, NodeToken,
	RawNodeToken, TokenToken,
};
use std::path::Path;
pub use tokens::Tokens;

#[derive(Default, Debug)]
pub struct FormatContext {
	pub tokens: Tokens,
}

/// This trait should be implemented on each node/value that should have a formatted representation
pub trait FormatValue {
	fn format(&self, context: &mut FormatContext) -> FormatToken;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IndentStyle {
	/// Tab
	Tab,
	/// Space, with its quantity
	Space(u8),
}

impl Default for IndentStyle {
	fn default() -> Self {
		Self::Tab
	}
}

impl FromStr for IndentStyle {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"tab" => Ok(Self::Tab),
			"space" => Ok(Self::Space(2)),
			// TODO: replace this error with a diagnostic
			_ => Err("Value not supported for IndentStyle"),
		}
	}
}

#[derive(Debug, Default)]
pub struct FormatOptions {
	/// The indent style
	indent_style: IndentStyle,
}

impl FormatOptions {
	pub fn new(indent_style: IndentStyle) -> Self {
		Self { indent_style }
	}
}
// TODO: implement me + handle errors
/// Main function
pub fn format(path: &Path, options: FormatOptions) {
	println!(
		"Running formatter to:\n- file {:?}\n- with options {:?}",
		path, options.indent_style
	);

	// we assume that file exists
	let mut file = File::open(&path).expect("cannot open the file to format");
	let mut buffer = String::new();
	// we assume we have permissions
	file.read_to_string(&mut buffer)
		.expect("cannot read the file to format");

	let result = format_str(buffer.as_str(), options);

	println!("{}", result.root().text());
}

pub fn format_str(content: &str, options: FormatOptions) -> PrintResult {
	let tokens = tokenize_json(content);
	format_token(&tokens, options)
}

pub fn format_token(token: &FormatToken, options: FormatOptions) -> PrintResult {
	let printer = Printer::new(options);
	printer.print(token)
}

#[cfg(test)]
mod tests {
	use crate::format_json::tokenize_json;
	use crate::{format_token, FormatOptions, FormatToken};
	use rslint_parser::ast::GroupingExpr;
	use rslint_parser::{parse_text, AstNode, SyntaxKind};

	#[test]
	fn formatting_a_formatted_node_returns_the_same_node() {
		let json = r#"[[1, null], [1, null], [null], [0], [false], [""]]\n"#;

		let tokens = tokenize_json(json);
		let formatted = format_token(&tokens, FormatOptions::default());

		let root_prev = match tokens {
			FormatToken::Node(node_token) => node_token.node,
			_ => panic!("Expected node"),
		};

		let script = parse_text(format!("({})", json).as_str(), 0);

		let grouping = GroupingExpr::cast(
			script
				.syntax()
				.descendants()
				.find(|e| GroupingExpr::can_cast(e.kind()))
				.unwrap(),
		)
		.unwrap();
		let json_content = grouping.inner().unwrap();

		assert_eq!(json_content.syntax().green(), formatted.root().green());

		assert!(json_content
			.syntax()
			.green()
			.shallow_eq(formatted.root().green()));
	}
}
