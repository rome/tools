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

	/// Deduce the file handler based on the extension of the file.
	///
	/// Any error will default to the base file handler for now.
	///
	///
	/// ```rust
	/// use path::RomePath;
	/// use core::create_app;
	/// let app = create_app();
	/// let file = RomePath::new("file.js").deduce_handler(&app);
	/// let handler = file.get_handler();
	/// let expected = app.get_js_handler("js").unwrap();
	/// assert_eq!(handler.unwrap(), expected)
	/// ```
	pub fn deduce_handler(mut self, app: &'handler App) -> Self {
		if let Some(extension) = self.extension() {
			if let Some(extension) = extension.to_str() {
				match extension {
					"js" => {
						let handler = app.get_js_handler(extension).unwrap();
						self.handler = Some(handler);
					}
					"json" => {
						let handler = app.get_json_handler(extension).unwrap();
						self.handler = Some(handler);
					}
					_ => self.handler = Some(app.get_base_handler()),
				}
			}
		} else {
			self.handler = Some(app.get_base_handler());
		}

		self
	}

	// TODO: handle error with diagnostic?
	/// Opens a file and returns a [File] in write mode
	pub fn open(&self) -> File {
		File::open(&self.file).expect("cannot open the file to format")
	}

	/// Returns the current handler associated to the file.
	///
	/// You need to call [deduce_handler] first in order to receive one. If not, [None] is always returned.
	pub fn get_handler(&self) -> Option<&FileHandlers> {
		self.handler
	}
}

#[cfg(test)]
mod test {
	use crate::RomePath;
	use core::create_app;

	#[test]
	fn deduce_handler() {
		let app = create_app();
		let file = RomePath::new("file.js").deduce_handler(&app);
		let handler = file.get_handler();
		let expected = app.get_js_handler("js").unwrap();
		assert_eq!(handler.unwrap(), expected)
	}
}
