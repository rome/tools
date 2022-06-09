use std::ffi::OsStr;

use rome_analyze::{AnalyzerAction, RuleCategories};
use rome_diagnostics::Diagnostic;
use rome_formatter::{IndentStyle, Printed};
use rome_fs::RomePath;
use rome_js_syntax::{JsLanguage, TextRange, TextSize};

use crate::{settings::SettingsHandle, workspace::server::AnyParse, RomeError};

use self::{javascript::JsFileHandler, json::JsonFileHandler, unknown::UnknownFileHandler};

mod javascript;
mod json;
mod unknown;

pub use javascript::JsFormatSettings;

/// Supported languages by Rome
#[derive(Debug, PartialEq)]
pub(crate) enum Language {
    /// JavaScript, TypeScript, JSX, TSX
    JavaScript,
    /// JSON
    Json,
    /// Any language that is not supported
    Unknown,
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" | "cts" | "mts" => Language::JavaScript,
            "json" => Language::Json,
            _ => Language::Unknown,
        }
    }
}

impl From<&OsStr> for Language {
    fn from(s: &OsStr) -> Self {
        match s.to_str().unwrap() {
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" | "cts" | "mts" => Language::JavaScript,
            "json" => Language::Json,
            _ => Language::Unknown,
        }
    }
}

// TODO: The Css variant is unused at the moment
#[allow(dead_code)]
pub(crate) enum Mime {
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

type Parse = fn(&RomePath, &str) -> AnyParse;
type DebugPrint = fn(&RomePath, AnyParse) -> String;
type Lint = fn(&RomePath, AnyParse, RuleCategories) -> Vec<Diagnostic>;
type CodeActions = fn(&RomePath, AnyParse, TextRange) -> Vec<AnalyzerAction<JsLanguage>>;
type Format = fn(&RomePath, AnyParse, SettingsHandle<IndentStyle>) -> Result<Printed, RomeError>;
type FormatRange =
    fn(&RomePath, AnyParse, SettingsHandle<IndentStyle>, TextRange) -> Result<Printed, RomeError>;
type FormatOnType =
    fn(&RomePath, AnyParse, SettingsHandle<IndentStyle>, TextSize) -> Result<Printed, RomeError>;

pub(crate) struct Capabilities {
    pub(crate) parse: Option<Parse>,
    pub(crate) debug_print: Option<DebugPrint>,
    pub(crate) lint: Option<Lint>,
    pub(crate) code_actions: Option<CodeActions>,
    pub(crate) format: Option<Format>,
    pub(crate) format_range: Option<FormatRange>,
    pub(crate) format_on_type: Option<FormatOnType>,
}

/// Main trait to use to add a new language to Rome
pub(crate) trait ExtensionHandler {
    /// The language of the file. It can be a super language.
    /// For example, a ".js" file can have [Language::Ts]
    fn language(&self) -> Language;

    /// MIME types used to identify a certain language
    fn mime(&self) -> Mime;

    /// A file that can support tabs inside its content
    fn may_use_tabs(&self) -> bool {
        true
    }

    /// Capabilities that can applied to a file
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parse: None,
            debug_print: None,
            format: None,
            lint: None,
            code_actions: None,
            format_range: None,
            format_on_type: None,
        }
    }

    /// How a file should be treated. Usually an asset doesn't posses a parser.
    ///
    /// An image should me parked as asset.
    fn is_asset(&self) -> bool {
        false
    }
}

/// Features available for each language
pub(crate) struct Features {
    js: JsFileHandler,
    json: JsonFileHandler,
    unknown: UnknownFileHandler,
}

impl Features {
    pub(crate) fn new() -> Self {
        Features {
            js: JsFileHandler {},
            json: JsonFileHandler {},
            unknown: UnknownFileHandler::default(),
        }
    }

    /// Return a [Language] from a string
    fn get_language(rome_path: &RomePath) -> Language {
        match rome_path.extension() {
            Some(file_extension) => file_extension.into(),
            None => Language::Unknown,
        }
    }

    /// Returns the [Capabilities] associated with a [RomePath]
    pub(crate) fn get_capabilities(&self, rome_path: &RomePath) -> Capabilities {
        match Self::get_language(rome_path) {
            Language::JavaScript => self.js.capabilities(),
            Language::Json => self.json.capabilities(),
            Language::Unknown => self.unknown.capabilities(),
        }
    }
}
