use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]

pub enum IndentStyle {
	Tab,
	Space,
}

impl FromStr for IndentStyle {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"tab" => Ok(Self::Tab),
			"space" => return Ok(Self::Space),
			// TODO: replace this error with a diagnostic
			_ => Err("Value not supported for IndentStyle"),
		}
	}
}

// TODO: evaluate to remove it
#[derive(Debug)]
pub struct FormatOptions {
	/// The style of the
	indent_style: IndentStyle,
	/// The size of the indentation
	indent_size: u8,
}

impl FormatOptions {
	pub fn new(style: &str, size: u8) -> Self {
		Self {
			indent_size: size,
			indent_style: IndentStyle::from_str(style).unwrap(),
		}
	}
}
// TODO: implement me
pub fn format(path: PathBuf, options: FormatOptions) {
	println!(
		"Running formatter to: \n- file {:?} \n- with options {:?}",
		path, options
	);
}
