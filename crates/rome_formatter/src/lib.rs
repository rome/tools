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
//! use rome_formatter::{format_elements, format_element, Formatter, ToFormatElement, FormatElement, FormatResult, FormatOptions, space_token, token };
//!
//! struct KeyValue {
//!     key: String,
//!     value: String
//! }
//!
//! impl ToFormatElement for KeyValue {
//!     fn to_format_element(&self, formatter: &Formatter)-> FormatResult<FormatElement>  {
//!         Ok(format_elements![
//!             token(self.key.as_str()),
//!             space_token(),
//!             token("=>"),
//!             space_token(),
//!             token(self.value.as_str())
//!         ])
//!     }
//! }
//!
//! fn my_function() {
//!     let key_value = KeyValue { key: String::from("lorem"), value: String::from("ipsum") };
//!     let element = key_value.to_format_element(&Formatter::default()).unwrap();
//!     let result = format_element(&element, FormatOptions::default());
//!     assert_eq!(result.code(), "lorem => ipsum");
//! }
//!
//! ```
//! [IR]: https://en.wikipedia.org/wiki/Intermediate_representation

mod cst;
mod format_element;
mod format_elements;
mod format_json;
mod formatter;
mod intersperse;
mod printer;
mod ts;

use crate::format_json::tokenize_json;

pub use formatter::Formatter;
use rslint_parser::SyntaxError;

pub use format_element::{
	block_indent, concat_elements, empty_element, group_elements, hard_line_break, if_group_breaks,
	if_group_fits_on_single_line, indent, join_elements, soft_indent, soft_line_break,
	soft_line_break_or_space, space_token, token, FormatElement,
};
pub use printer::Printer;
pub use printer::PrinterOptions;
use rome_core::file_handlers::Language;
use rome_core::App;
use rome_path::RomePath;
use rslint_parser::parse_text;

use std::io::Read;
use std::str::FromStr;

/// This trait should be implemented on each node/value that should have a formatted representation
pub trait ToFormatElement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

/// Public return type of the formatter
pub type FormatResult<F> = Result<F, FormatError>;

#[derive(Debug, PartialEq)]
/// Series of errors encountered during formatting
pub enum FormatError {
	/// Node is missing and it should be required for a correct formatting
	MissingRequiredChild,

	/// In case our formatter doesn't know how to format a certain language
	UnsupportedLanguage,

	/// When the ability to format the current file has been turned off on purpose
	CapabilityDisabled,
}

impl From<SyntaxError> for FormatError {
	fn from(syntax_error: SyntaxError) -> Self {
		match syntax_error {
			SyntaxError::MissingRequiredChild(_node) => FormatError::MissingRequiredChild,
		}
	}
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
pub struct Formatted {
	code: String,
}

impl Formatted {
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
pub fn format(rome_path: &mut RomePath, options: FormatOptions) -> FormatResult<Formatted> {
	// we assume that file exists
	let mut file = rome_path.open();
	let mut buffer = String::new();
	// we assume we have permissions
	file.read_to_string(&mut buffer)
		.expect("cannot read the file to format");

	if let Some(handler) = rome_path.get_handler() {
		if handler.capabilities().format {
			let result = match handler.language() {
				Language::Js => {
					let parsed_result = parse_text(buffer.as_str(), 0);
					Formatter::new(options).format_root(&parsed_result.syntax())
				}
				Language::Json => {
					let element = tokenize_json(buffer.as_str());
					Ok(format_element(&element, options))
				}
				Language::Ts | Language::Unknown => Err(FormatError::UnsupportedLanguage),
			};

			result
		} else {
			Err(FormatError::CapabilityDisabled)
		}
	} else {
		Err(FormatError::UnsupportedLanguage)
	}
}

pub fn format_file_and_save(rome_path: &mut RomePath, options: FormatOptions) {
	let result = format(rome_path, options);
	if let Ok(result) = result {
		rome_path
			.save(result.code())
			.expect("Could not write the formatted code on file");
	}
}

pub fn format_file(path_to_file: &str, options: FormatOptions, app: &App) -> Formatted {
	let mut rome_path = RomePath::new(path_to_file).deduce_handler(app);
	let element = format(&mut rome_path, options);
	element.unwrap()
}

pub fn format_element(element: &FormatElement, options: FormatOptions) -> Formatted {
	let printer = Printer::new(options);
	printer.print(element)
}
