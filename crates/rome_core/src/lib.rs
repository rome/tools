use crate::file_handlers::javascript::JsFileFeatures;
use crate::file_handlers::json::JsonFileFeatures;
use crate::file_handlers::unknown::UnknownFileHandler;
use crate::file_handlers::{javascript::JsFileHandler, ExtensionHandler, Language};
use file_handlers::json::JsonFileHandler;
use rome_path::RomePath;
use std::collections::HashMap;

pub mod file_handlers;

struct SupportedLanguages {
    js: JsFileHandler,
    ts: JsFileHandler,
    json: JsonFileHandler,
    unknown: UnknownFileHandler,
}

pub struct App {
    supported_languages: SupportedLanguages,
    js_files: HashMap<RomePath, JsFileFeatures>,
    json_files: HashMap<RomePath, JsonFileFeatures>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            supported_languages: SupportedLanguages {
                js: JsFileHandler {},
                ts: JsFileHandler {},
                json: JsonFileHandler {},
                unknown: UnknownFileHandler::default(),
            },
            js_files: HashMap::new(),
            json_files: HashMap::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn store_js_file(&mut self, path_to_file: &str, module: bool) {
        let path = RomePath::new(path_to_file);
        let features = if module {
            JsFileFeatures::module()
        } else {
            JsFileFeatures::script()
        };
        self.js_files.insert(path, features);
    }

    pub fn get_js_file(&self, path: &RomePath) -> Option<&JsFileFeatures> {
        self.js_files.get(path)
    }

    pub fn store_json_file(&mut self, path_to_file: &str) {
        let path = RomePath::new(path_to_file);
        let features = JsonFileFeatures::default();
        self.json_files.insert(path, features);
    }

    pub fn get_json_file(&self, path: &RomePath) -> Option<&JsonFileFeatures> {
        self.json_files.get(path)
    }

    pub fn get_language<L: Into<Language>>(&self, file_extension: L) -> Language {
        file_extension.into()
    }

    pub fn is_language_supported<L: Into<Language>>(&self, file_extension: L) -> bool {
        Language::Unknown != file_extension.into()
    }

    pub fn get_js_features(&self) -> &JsFileHandler {
        &self.supported_languages.js
    }

    pub fn get_ts_features(&self) -> &JsFileHandler {
        &self.supported_languages.ts
    }

    pub fn get_json_features(&self) -> &JsonFileHandler {
        &self.supported_languages.json
    }

    pub fn get_unknown_features(&self) -> &UnknownFileHandler {
        &self.supported_languages.unknown
    }
}
