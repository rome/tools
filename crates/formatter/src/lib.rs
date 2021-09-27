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
//! use rome_formatter::{format_tokens, format_token, FormatToken, FormatValue, FormatOptions};
//!
//! struct KeyValue {
//!     key: String,
//!     value: String
//! }
//!
//! impl FormatValue for KeyValue {
//!     fn format(&self) -> FormatToken {
//!         format_tokens!(self.key.as_str(), FormatToken::Space, "=>", FormatToken::Space, self.value.as_str())
//!     }
//! }
//!
//! fn main() {
//!	    let key_value = KeyValue { key: String::from("lorem"), value: String::from("ipsum") };
//!     let token = key_value.format();
//!     let options = FormatOptions::default();
//!     let result = format_token(&token, options);
//!     assert_eq!(result.code(), "lorem => ipsum");
//! }
//!
//! ```
//! [IR]: https://en.wikipedia.org/wiki/Intermediate_representation

mod format_json;
mod format_token;
mod format_token_macro;
mod intersperse;
mod printer;

use crate::format_json::json_to_tokens;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

pub use format_token::{FormatToken, GroupToken, IfBreakToken, IndentToken, LineMode, ListToken, LineToken};
pub use printer::PrintResult;
pub use printer::Printer;
pub use printer::PrinterOptions;

/// This trait should be implemented on each node/value that should have a formatted representation
pub trait FormatValue {
	fn format(&self) -> FormatToken;
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
pub fn format(path: PathBuf, options: FormatOptions) {
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

	let tokens = json_to_tokens(buffer.as_str());
	let print_result = format_token(&tokens, options);

	println!("{}", print_result.code());
}

pub fn format_str(content: &str, options: FormatOptions) -> PrintResult {
	let tokens = json_to_tokens(content);
	format_token(&tokens, options)
}

pub fn format_token(token: &FormatToken, options: FormatOptions) -> PrintResult {
	let printer = Printer::new(options);
	printer.print(token)
}
