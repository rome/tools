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
//! use rome_formatter::{format_tokens, format_element, FormatToken, FormatValue, FormatOptions, space_token, token};
//!
//! struct KeyValue {
//!     key: String,
//!     value: String
//! }
//!
//! impl FormatValue for KeyValue {
//!     fn format(&self) -> FormatToken {
//!         format_tokens![token(self.key.as_str()), space_token(), token("=>"), space_token(), token(self.value.as_str())]
//!     }
//! }
//!
//! fn my_function() {
//!     let key_value = KeyValue { key: String::from("lorem"), value: String::from("ipsum") };
//!     let element = key_value.format();
//!     let result = format_element(&element, FormatOptions::default());
//!     assert_eq!(result.code(), "lorem => ipsum");
//! }
//!
//! ```
//! [IR]: https://en.wikipedia.org/wiki/Intermediate_representation

mod format_json;
mod format_token;
mod format_tokens_macro;
mod intersperse;
mod printer;

use crate::format_json::tokenize_json;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

pub use format_token::{
	concat_elements, group_elements, hard_line_break, if_group_breaks,
	if_group_fits_on_single_line, indent, join_elements, soft_indent, soft_line_break,
	soft_line_break_or_space, space_token, token, FormatToken,
};
use printer::Printer;

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

#[derive(Debug)]
pub struct FormatOptions {
	/// The indent style
	pub indent_style: IndentStyle,

	/// What's the max width of a line. Defaults to 80
	pub line_width: u16,
}

impl FormatOptions {
	pub fn new(indent_style: IndentStyle) -> Self {
		Self {
			indent_style,
			..Self::default()
		}
	}
}

impl Default for FormatOptions {
	fn default() -> Self {
		Self {
			indent_style: IndentStyle::default(),
			line_width: 80,
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatResult {
	code: String,
}

impl FormatResult {
	pub fn new(code: &str) -> Self {
		Self {
			code: String::from(code),
		}
	}

	pub fn code(&self) -> &String {
		&self.code
	}
}

// TODO: implement me + handle errors
/// Main function
pub fn format(path: PathBuf, options: FormatOptions) -> FormatResult {
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

	let tokens = tokenize_json(buffer.as_str());
	let result = format_element(&tokens, options);

	println!("{}", result.code());

	result
}

pub fn format_str(content: &str, options: FormatOptions) -> FormatResult {
	let tokens = tokenize_json(content);
	format_element(&tokens, options)
}

pub fn format_element(element: &FormatToken, options: FormatOptions) -> FormatResult {
	let printer = Printer::new(options);
	printer.print(element)
}
