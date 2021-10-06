use super::ExtensionHandler;

pub struct BaseFileHandler {}

impl ExtensionHandler for BaseFileHandler {
	fn capabilities(&self) -> super::Capabilities {
		super::Capabilities {
			format: false,
			lint: false,
		}
	}

	fn language(&self) -> super::Language {
		super::Language::Unknown
	}

	fn mime(&self) -> super::Mime {
		super::Mime::Text
	}

	fn use_tabs(&self) -> bool {
		true
	}
}
