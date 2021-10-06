//! This module is responsible to manage paths inside Rome.
//! It is a small wrapper around [path::PathBuf] but it is also able to
//! give additional information around the the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use core::{file_handlers::FileHandlers, App};
use std::{fmt::Debug, fs::File, ops::Deref, path::PathBuf};

pub struct RomePath<'handler> {
	file: PathBuf,

	handler: Option<&'handler FileHandlers>,
}

impl<'handler> Deref for RomePath<'handler> {
	type Target = PathBuf;

	fn deref(&self) -> &Self::Target {
		&self.file
	}
}

impl<'handler> Debug for RomePath<'handler> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl<'handler> RomePath<'handler> {
	pub fn new(path_to_file: &str) -> Self {
		Self {
			file: PathBuf::from(path_to_file),
			handler: None,
		}
	}

	/// deduce the handler based on the extension of the file
	pub fn deduce_handler(mut self, app: &'handler App) -> Result<Self, String> {
		if let Some(extension) = self.extension() {
			let extension = extension.to_str().unwrap();
			match extension {
				"js" => {
					let handler = app.get_js_handler(extension).unwrap();
					self.handler = Some(handler);
					return Ok(self);
				}
				"json" => {
					let handler = app.get_json_handler(extension).unwrap();
					self.handler = Some(handler);
					return Ok(self);
				}
				_ => return Err(format!("We don't have {} stored in Rome", extension)),
			}
		}
		Err(format!("We don't have  stored in Rome"))
	}

	pub fn open(&self) -> File {
		File::open(&self.file).expect("cannot open the file to format")
	}

	pub fn get_handler(&self) -> Option<&FileHandlers> {
		self.handler
	}
}
