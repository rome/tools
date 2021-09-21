mod format_node;
mod intersperse;
mod token;

use format_node::FormatValue;
use serde_json::Value;
pub use token::Token;

use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

#[derive(Debug)]

pub enum IndentStyle {
	/// Tab
	Tab,
	/// Space, with its quantity
	Space(u8),
}

impl FromStr for IndentStyle {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"tab" => Ok(Self::Tab),
			"space" => return Ok(Self::Space(2)),
			// TODO: replace this error with a diagnostic
			_ => Err("Value not supported for IndentStyle"),
		}
	}
}

// TODO: evaluate to remove it
#[derive(Debug)]
pub struct FormatOptions {
	/// The indent style
	indent_style: IndentStyle,
}

impl FormatOptions {
	pub fn new(indent_style: IndentStyle) -> Self {
		Self { indent_style }
	}
}
// TODO: implement me
/// Main function
pub fn format(path: PathBuf, options: FormatOptions) {
	println!(
		"Running formatter to: \n- file {:?} \n- with options {:?}",
		path, options
	);
	// we assume that file exists
	let mut file = File::open(&path).unwrap();
	let mut buffer = String::new();
	// we assume we have permissions
	file.read_to_string(&mut buffer).unwrap();
	let json: Value = serde_json::from_str(buffer.as_str()).unwrap();

	let tokens = json.format();

	println!("{:?}", tokens);
}
