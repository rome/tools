use crate::file_handlers::{unknown::UnknownFileHandler, javascript::JsFileHandler};
use file_handlers::{json::JsonFileHandler, ExtensionHandler};
use std::collections::HashMap;

pub mod file_handlers;

// these strings will live for the whole App, so it makes sense to have them as static
pub type Handlers = HashMap<&'static str, Box<dyn ExtensionHandler>>;

pub struct App {
	handlers: Handlers,
}

impl Default for App {
	fn default() -> Self {
		let mut map: Handlers = HashMap::new();
		map.insert("js", Box::new(JsFileHandler {}));
		map.insert("jsx", Box::new(JsFileHandler {}));
		map.insert("ts", Box::new(JsFileHandler {}));
		map.insert("tsx", Box::new(JsFileHandler {}));
		map.insert("json", Box::new(JsonFileHandler {}));
		map.insert("unknown", Box::new(UnknownFileHandler {}));
		Self { handlers: map }
	}
}

#[allow(clippy::borrowed_box)]
impl App {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn get_handler<'a>(&self, file_extension: &'a str) -> Option<&Box<dyn ExtensionHandler>> {
		if self.handlers.contains_key(file_extension) {
			self.handlers.get(file_extension)
		} else {
			None
		}
	}
}

pub fn create_app() -> App {
	App::new()
}
