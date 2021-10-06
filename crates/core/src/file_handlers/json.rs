use super::{ExtensionHandler, Mime};

pub struct JsonFileHandler {}

impl ExtensionHandler for JsonFileHandler {
	fn capabilities(&self) -> super::Capabilities {
		super::Capabilities {
			format: true,
			lint: true,
		}
	}

	fn language(&self) -> super::Language {
		super::Language::Json
	}

	fn mime(&self) -> super::Mime {
		Mime::Json
	}

	fn use_tabs(&self) -> bool {
		true
	}
}
