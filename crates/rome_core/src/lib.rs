use crate::file_handlers::unknown::UnknownFileHandler;
use crate::file_handlers::{javascript::JsFileHandler, ExtensionHandler, Language};
use file_handlers::json::JsonFileHandler;
use rome_path::RomePath;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            features: Features {
                js: JsFileHandler {},
                json: JsonFileHandler {},
                unknown: UnknownFileHandler::default(),
            },
        }
    }
}

/// Generic errors thrown during rome operations
pub enum RomeError {
    /// A file can't be read
    CantReadTheFile,
    /// A file is not supported. It contains the extension of the file
    /// Use this error if Rome is trying to process a file that Rome can't understand
    SourceFileNotSupported(String),
}

impl Debug for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RomeError::SourceFileNotSupported(_) => std::fmt::Display::fmt(self, f),
            RomeError::CantReadTheFile => std::fmt::Display::fmt(self, f),
        }
    }
}

impl Display for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RomeError::SourceFileNotSupported(extension) => {
                write!(f, "Rome doesn't support this {extension} yet")
            }
            RomeError::CantReadTheFile => {
                write!(f, "Rome is not able to read the file")
            }
        }
    }
}

impl Error for RomeError {}

impl App {
    pub fn new() -> Self {
        Default::default()
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

    /// Checks if the current file can be formatted
    pub fn can_format(&self, rome_path: &RomePath) -> bool {
        rome_path.extension().map_or(false, |extension| {
            let language = self.get_language(extension);

            match language {
                Language::JavaScript => self.features.js.capabilities().format,
                Language::Json => self.features.json.capabilities().format,
                Language::Unknown => self.features.unknown.capabilities().format,
            }
        })
    }

    /// Checks if the current file can be analyzed for linting rules
    pub fn can_lint(&self, rome_path: &RomePath) -> bool {
        rome_path.extension().map_or(false, |extension| {
            let language = self.get_language(extension);

            match language {
                Language::JavaScript => self.features.js.capabilities().lint,
                Language::Json => self.features.json.capabilities().lint,
                Language::Unknown => self.features.unknown.capabilities().lint,
            }
        })
    }
}
