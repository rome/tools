//! This module is responsible to manage paths inside Rome.
//! It is a small wrapper around [path::PathBuf] but it is also able to
//! give additional information around the the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use rome_core::{file_handlers::ExtensionHandler, App};
use std::{fs::File, io::Write, ops::Deref, path::PathBuf};

pub struct RomePath<'handler> {
	file: PathBuf,
	handler: Option<&'handler dyn ExtensionHandler>,
}

impl<'handler> Deref for RomePath<'handler> {
	type Target = PathBuf;

	fn deref(&self) -> &Self::Target {
		&self.file
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
	/// use rome_core::{
	///   create_app,
	///   file_handlers::{javascript::JsFileHandler, ExtensionHandler},
	/// };
	///
	/// let app = create_app();
	/// let file = RomePath::new("file.js").deduce_handler(&app);
	/// let handler = file.get_handler();
	/// let expected = JsFileHandler {};
	/// assert_eq!(
	///   handler.unwrap().capabilities().format,
	///   expected.capabilities().format
	/// );
	/// assert_eq!(
	///  handler.unwrap().capabilities().lint,
	///  expected.capabilities().lint
	/// )
	/// ```
	pub fn deduce_handler(mut self, app: &'handler App) -> Self {
		if self.extension().is_none() {
			return self;
		}
		let extension = self.extension().unwrap().to_str().unwrap();

		if let Some(handler) = app.get_handler(extension) {
			self.handler = Some(handler);
		} else {
			// we know that unknown is hardcoded
			self.handler = Some(app.get_handler("unknown").unwrap());
		}

		self
	}

	// TODO: handle error with diagnostic?
	/// Opens a file and returns a [File] in write mode
	pub fn open(&self) -> File {
		File::open(&self.file).expect("cannot open the file to format")
	}

	/// Accepts a file opened in read mode and saves into it
	pub fn save(&mut self, content: &str) -> Result<(), std::io::Error> {
		let mut file_to_write = File::create(&self.file).unwrap();
		// TODO: handle error with diagnostic
		file_to_write.write_all(content.as_bytes())
	}

	/// Returns the current handler associated to the file.
	///
	/// You need to call [deduce_handler] first in order to receive one. If not, [None] is always returned.
	// TODO: move handler deduction inside the path crate
	pub fn get_handler(&self) -> Option<&dyn ExtensionHandler> {
		self.handler
	}
}

#[cfg(test)]
mod test {
	use crate::RomePath;
	use rome_core::{
		create_app,
		file_handlers::{javascript::JsFileHandler, ExtensionHandler},
	};

	#[test]
	fn deduce_handler() {
		let app = create_app();
		let file = RomePath::new("file.js").deduce_handler(&app);
		let handler = file.get_handler();
		let expected = JsFileHandler {};
		assert_eq!(
			handler.unwrap().capabilities().format,
			expected.capabilities().format
		);
		assert_eq!(
			handler.unwrap().capabilities().lint,
			expected.capabilities().lint
		)
	}
}
