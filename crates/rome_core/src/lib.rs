use crate::file_handlers::javascript::JsFileFeatures;
use crate::file_handlers::json::JsonFileFeatures;
use crate::file_handlers::unknown::UnknownFileHandler;
use crate::file_handlers::{javascript::JsFileHandler, ExtensionHandler, Language};
use file_handlers::json::JsonFileHandler;
use rome_path::RomePath;
use std::collections::HashMap;

pub mod file_handlers;

/// Features available for each language
struct Features {
    js: JsFileHandler,
    json: JsonFileHandler,
    unknown: UnknownFileHandler,
}

pub struct App {
    /// features available throughout the application
    features: Features,
    /// Associate a JavaScript file with its features
    js_files: HashMap<RomePath, JsFileFeatures>,
    /// Associate a JSON file with its features
    json_files: HashMap<RomePath, JsonFileFeatures>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            features: Features {
                js: JsFileHandler {},
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

    /// Store a JavaScript file with its features
    pub fn store_js_file(&mut self, path_to_file: &str, file_features: JsFileFeatures) {
        let path = RomePath::new(path_to_file);
        self.js_files.insert(path, file_features);
    }

    /// Return the [JsFileFeatures] that belong to the given file
    pub fn get_js_file(&self, path: &RomePath) -> Option<&JsFileFeatures> {
        self.js_files.get(path)
    }

    /// Store a JSON file with its features
    pub fn store_json_file(&mut self, path_to_file: &str) {
        let path = RomePath::new(path_to_file);
        let features = JsonFileFeatures::default();
        self.json_files.insert(path, features);
    }

    /// Return the [JsonFileFeatures] that belong to the given file
    pub fn get_json_file(&self, path: &RomePath) -> Option<&JsonFileFeatures> {
        self.json_files.get(path)
    }

    /// Return a [Language] from a string
    pub fn get_language<L: Into<Language>>(&self, file_extension: L) -> Language {
        file_extension.into()
    }

    /// Check if the current language is supported
    pub fn is_language_supported<L: Into<Language>>(&self, file_extension: L) -> bool {
        Language::Unknown != file_extension.into()
    }

    /// Return the features that are available for JavaScript
    pub fn get_js_features(&self) -> &JsFileHandler {
        &self.features.js
    }

    /// Return the features that are available for JSON
    pub fn get_json_features(&self) -> &JsonFileHandler {
        &self.features.json
    }

    /// Features available to a language that is not supported
    pub fn get_unknown_features(&self) -> &UnknownFileHandler {
        &self.features.unknown
    }
}
