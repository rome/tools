use super::{ExtensionHandler, Mime};

#[derive(Debug, PartialEq, Eq)]
pub struct JsFileHandler {}

impl ExtensionHandler for JsFileHandler {
	fn capabilities(&self) -> super::Capabilities {
		super::Capabilities {
			format: true,
			lint: true,
		}
	}

	fn language(&self) -> super::Language {
		super::Language::Js
	}

	fn mime(&self) -> super::Mime {
		Mime::Javascript
	}

	fn use_tabs(&self) -> bool {
		true
	}
}
