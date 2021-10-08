pub mod javascript;
pub mod json;
pub mod unknown;
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

#[derive(Debug)]
pub struct Capabilities {
	pub lint: bool,
	pub format: bool,
}

/// Main trait to use to add a new language to Rome
pub trait ExtensionHandler {
	fn language(&self) -> Language;

	fn mime(&self) -> Mime;

	fn may_use_tabs(&self) -> bool {
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
}
