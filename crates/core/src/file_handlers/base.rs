use super::ExtensionHandler;
#[derive(Debug, PartialEq, Eq)]
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

	fn may_use_tabs(&self) -> bool {
		true
	}
}
