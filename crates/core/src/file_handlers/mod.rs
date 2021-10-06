use self::{base::BaseFileHandler, javascript::JsFileHandler, json::JsonFileHandler};

pub mod base;
pub mod javascript;
pub mod json;
pub enum Language {
	Js,
	Json,
	Unknown,
}

pub enum Mime {
	Javascript,
	Json,
	Css,
	Text,
}

impl std::fmt::Display for Mime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Mime::Css => write!(f, "text/css"),
			Mime::Json => write!(f, "application/json"),
			Mime::Javascript => write!(f, "application/javascript"),
			Mime::Text => write!(f, "text/plain"),
		}
	}
}

pub struct Capabilities {
	pub lint: bool,
	pub format: bool,
}

/// Main trait to use to add a new language to Rome
pub trait ExtensionHandler {
	fn language(&self) -> Language;

	fn mime(&self) -> Mime;

	fn use_tabs(&self) -> bool {
		true
	}

	fn capabilities(&self) -> Capabilities {
		Capabilities {
			format: false,
			lint: false,
		}
	}

	fn is_asset(&self) -> bool {
		false
	}

	// TODO: what is this?
	fn can_have_scale(&self) -> bool {
		false
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileHandlers {
	Js(JsFileHandler),
	Json(JsonFileHandler),
	Base(BaseFileHandler),
}
