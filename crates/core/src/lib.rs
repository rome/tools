use crate::file_handlers::{base::BaseFileHandler, javascript::JsFileHandler};
use file_handlers::{json::JsonFileHandler, FileHandlers};

pub mod file_handlers;

pub struct App {
	js_file_handler: FileHandlers,
	base_file_handler: FileHandlers,
	json_file_handler: FileHandlers,
}

impl App {
	pub fn new() -> Self {
		Self {
			js_file_handler: FileHandlers::Js(JsFileHandler {}),
			base_file_handler: FileHandlers::Base(BaseFileHandler {}),
			json_file_handler: FileHandlers::Json(JsonFileHandler {}),
		}
	}

	pub fn get_js_handler<'a>(&self, file_extension: &'a str) -> Option<&FileHandlers> {
		match file_extension {
			"js" | "ts" | "tsx" | "jsx" => Some(&self.js_file_handler),
			_ => None,
		}
	}

	pub fn get_json_handler<'a>(&self, file_extension: &'a str) -> Option<&FileHandlers> {
		match file_extension {
			"json" => Some(&self.json_file_handler),
			_ => None,
		}
	}
}

pub fn create_app() -> App {
	let app = App::new();
	app
}
