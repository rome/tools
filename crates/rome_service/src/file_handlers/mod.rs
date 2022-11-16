use self::{javascript::JsFileHandler, json::JsonFileHandler, unknown::UnknownFileHandler};
use crate::workspace::FixFileMode;
use crate::{
    settings::SettingsHandle,
    workspace::{
        server::AnyParse, FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult,
    },
    RomeError, Rules,
};
pub use javascript::JsFormatterSettings;
use rome_analyze::AnalysisFilter;
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};
use std::ffi::OsStr;

mod javascript;
mod json;
mod unknown;

/// Supported languages by Rome
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Language {
    /// JavaScript
    JavaScript,
    /// JSX
    JavaScriptReact,
    /// TypeScript
    TypeScript,
    /// TSX
    TypeScriptReact,
    /// JSON
    Json,
    /// Any language that is not supported
    #[default]
    Unknown,
}

impl Language {
    /// Returns the language corresponding to this file extension
    pub fn from_extension(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "jsx" => Language::JavaScriptReact,
            "ts" | "mts" | "cts" => Language::TypeScript,
            "tsx" => Language::TypeScriptReact,
            "json" => Language::Json,
            _ => Language::Unknown,
        }
    }

    /// Returns the language corresponding to this language ID
    ///
    /// See the [microsoft spec] <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem>
    /// for a list of language identifiers
    ///
    /// [microsoft spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    pub fn from_language_id(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "javascript" => Language::JavaScript,
            "typescript" => Language::TypeScript,
            "javascriptreact" => Language::JavaScriptReact,
            "typescriptreact" => Language::TypeScriptReact,
            "json" => Language::Json,
            _ => Language::Unknown,
        }
    }

    /// Returns the language if it's not unknown, otherwise returns `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rome_service::workspace::Language;
    /// let x = Language::JavaScript;
    /// let y = Language::Unknown;
    /// assert_eq!(x.or(y), Language::JavaScript);
    ///
    /// let x = Language::Unknown;
    /// let y = Language::JavaScript;
    /// assert_eq!(x.or(y), Language::JavaScript);
    ///
    /// let x = Language::JavaScript;
    /// let y = Language::Json;
    /// assert_eq!(x.or(y), Language::JavaScript);
    ///
    /// let x = Language::Unknown;
    /// let y = Language::Unknown;
    /// assert_eq!(x.or(y), Language::Unknown);
    /// ```
    pub fn or(self, other: Language) -> Language {
        if self != Language::Unknown {
            self
        } else {
            other
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

pub struct FixAllParams<'a> {
    pub(crate) rome_path: &'a RomePath,
    pub(crate) parse: AnyParse,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: SettingsHandle<'a>,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub(crate) struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
}

type Parse = fn(&RomePath, Language, &str) -> AnyParse;

#[derive(Default)]
pub(crate) struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,
}

type DebugSyntaxTree = fn(&RomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(&RomePath, AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(&RomePath, AnyParse, SettingsHandle) -> Result<String, RomeError>;

#[derive(Default)]
pub(crate) struct DebugCapabilities {
    /// Prints the syntax tree
    pub(crate) debug_syntax_tree: Option<DebugSyntaxTree>,
    /// Prints the control flow graph
    pub(crate) debug_control_flow: Option<DebugControlFlow>,
    /// Prints the formatter IR
    pub(crate) debug_formatter_ir: Option<DebugFormatterIR>,
}

pub(crate) struct LintParams<'a> {
    pub(crate) rome_path: &'a RomePath,
    pub(crate) parse: AnyParse,
    pub(crate) filter: AnalysisFilter<'a>,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) settings: SettingsHandle<'a>,
    pub(crate) max_diagnostics: u64,
}

pub(crate) struct LintResults {
    pub(crate) diagnostics: Vec<rome_diagnostics::v2::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u64,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions =
    fn(&RomePath, AnyParse, TextRange, Option<&Rules>, SettingsHandle) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, RomeError>;
type Rename = fn(&RomePath, AnyParse, TextSize, String) -> Result<RenameResult, RomeError>;

#[derive(Default)]
pub(crate) struct AnalyzerCapabilities {
    /// It lints a file
    pub(crate) lint: Option<Lint>,
    /// It extracts code actions for a file
    pub(crate) code_actions: Option<CodeActions>,
    /// Applies fixes to a file
    pub(crate) fix_all: Option<FixAll>,
    /// It renames a binding inside a file
    pub(crate) rename: Option<Rename>,
}

type Format = fn(&RomePath, AnyParse, SettingsHandle) -> Result<Printed, RomeError>;
type FormatRange = fn(&RomePath, AnyParse, SettingsHandle, TextRange) -> Result<Printed, RomeError>;
type FormatOnType = fn(&RomePath, AnyParse, SettingsHandle, TextSize) -> Result<Printed, RomeError>;

#[derive(Default)]
pub(crate) struct FormatterCapabilities {
    /// It formats a file
    pub(crate) format: Option<Format>,
    /// It formats a portion of text of a file
    pub(crate) format_range: Option<FormatRange>,
    /// It formats a file while typing
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
        Capabilities::default()
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
    pub(crate) fn get_language(rome_path: &RomePath) -> Language {
        rome_path
            .extension()
            .and_then(OsStr::to_str)
            .map(Language::from_extension)
            .unwrap_or_default()
    }

    /// Returns the [Capabilities] associated with a [RomePath]
    pub(crate) fn get_capabilities(
        &self,
        rome_path: &RomePath,
        language_hint: Language,
    ) -> Capabilities {
        match Self::get_language(rome_path).or(language_hint) {
            Language::JavaScript
            | Language::JavaScriptReact
            | Language::TypeScript
            | Language::TypeScriptReact => self.js.capabilities(),
            Language::Json => self.json.capabilities(),
            Language::Unknown => self.unknown.capabilities(),
        }
    }
}
