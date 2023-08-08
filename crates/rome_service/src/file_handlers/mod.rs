use self::{javascript::JsFileHandler, json::JsonFileHandler, unknown::UnknownFileHandler};
use crate::workspace::{FixFileMode, OrganizeImportsResult};
use crate::{
    settings::SettingsHandle,
    workspace::{FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult},
    Rules, WorkspaceError,
};
pub use javascript::JsFormatterSettings;
use rome_analyze::{AnalysisFilter, AnalyzerDiagnostic};
use rome_console::fmt::Formatter;
use rome_console::markup;
use rome_diagnostics::{Diagnostic, Severity};
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};
use rome_parser::AnyParse;
use rome_rowan::NodeCache;
use std::ffi::OsStr;
use std::path::Path;

mod javascript;
mod json;
mod unknown;

/// Supported languages by Rome
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
    /// JSONC
    Jsonc,
    /// Any language that is not supported
    #[default]
    Unknown,
}

impl Language {
    /// Files that can be bypassed, because correctly handled by the JSON parser
    pub(crate) const ALLOWED_FILES: &'static [&'static str; 15] = &[
        "typescript.json",
        "tsconfig.json",
        "jsconfig.json",
        "tslint.json",
        "babel.config.json",
        ".babelrc.json",
        ".ember-cli",
        "typedoc.json",
        ".eslintrc.json",
        ".eslintrc",
        ".jsfmtrc",
        ".jshintrc",
        ".swcrc",
        ".hintrc",
        ".babelrc",
    ];

    /// Returns the language corresponding to this file extension
    pub fn from_extension(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "jsx" => Language::JavaScriptReact,
            "ts" | "mts" | "cts" => Language::TypeScript,
            "tsx" => Language::TypeScriptReact,
            "json" => Language::Json,
            "jsonc" => Language::Jsonc,
            _ => Language::Unknown,
        }
    }

    pub fn from_known_filename(s: &str) -> Self {
        if Self::ALLOWED_FILES.contains(&s.to_lowercase().as_str()) {
            Language::Jsonc
        } else {
            Language::Unknown
        }
    }

    /// Returns the language corresponding to the file path
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|path| path.to_str())
            .map(Language::from_extension)
            .unwrap_or(Language::Unknown)
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
            "jsonc" => Language::Jsonc,
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

impl rome_console::fmt::Display for Language {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Language::JavaScript => fmt.write_markup(markup! { "JavaScript" }),
            Language::JavaScriptReact => fmt.write_markup(markup! { "JSX" }),
            Language::TypeScript => fmt.write_markup(markup! { "TypeScript" }),
            Language::TypeScriptReact => fmt.write_markup(markup! { "TSX" }),
            Language::Json => fmt.write_markup(markup! { "JSON" }),
            Language::Jsonc => fmt.write_markup(markup! { "JSONC" }),
            Language::Unknown => fmt.write_markup(markup! { "Unknown" }),
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

impl rome_console::fmt::Display for Mime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::io::Result<()> {
        write!(f, "{self}")
    }
}

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: SettingsHandle<'a>,
    /// Whether it should format the code action
    pub(crate) should_format: bool,
    pub(crate) rome_path: &'a RomePath,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
}

type Parse = fn(&RomePath, Language, &str, SettingsHandle, &mut NodeCache) -> AnyParse;

#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,
}

type DebugSyntaxTree = fn(&RomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(&RomePath, AnyParse, SettingsHandle) -> Result<String, WorkspaceError>;

#[derive(Default)]
pub struct DebugCapabilities {
    /// Prints the syntax tree
    pub(crate) debug_syntax_tree: Option<DebugSyntaxTree>,
    /// Prints the control flow graph
    pub(crate) debug_control_flow: Option<DebugControlFlow>,
    /// Prints the formatter IR
    pub(crate) debug_formatter_ir: Option<DebugFormatterIR>,
}

pub(crate) struct LintParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) filter: AnalysisFilter<'a>,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) settings: SettingsHandle<'a>,
    pub(crate) max_diagnostics: u64,
    pub(crate) path: &'a RomePath,
}

pub(crate) struct LintResults {
    pub(crate) diagnostics: Vec<rome_diagnostics::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u64,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions =
    fn(AnyParse, TextRange, Option<&Rules>, SettingsHandle, &RomePath) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, WorkspaceError>;
type Rename = fn(&RomePath, AnyParse, TextSize, String) -> Result<RenameResult, WorkspaceError>;
type OrganizeImports = fn(AnyParse) -> Result<OrganizeImportsResult, WorkspaceError>;

#[derive(Default)]
pub struct AnalyzerCapabilities {
    /// It lints a file
    pub(crate) lint: Option<Lint>,
    /// It extracts code actions for a file
    pub(crate) code_actions: Option<CodeActions>,
    /// Applies fixes to a file
    pub(crate) fix_all: Option<FixAll>,
    /// It renames a binding inside a file
    pub(crate) rename: Option<Rename>,
    /// It organize imports
    pub(crate) organize_imports: Option<OrganizeImports>,
}

type Format = fn(&RomePath, AnyParse, SettingsHandle) -> Result<Printed, WorkspaceError>;
type FormatRange =
    fn(&RomePath, AnyParse, SettingsHandle, TextRange) -> Result<Printed, WorkspaceError>;
type FormatOnType =
    fn(&RomePath, AnyParse, SettingsHandle, TextSize) -> Result<Printed, WorkspaceError>;

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
            .or(rome_path
                .file_name()
                .and_then(OsStr::to_str)
                .map(Language::from_known_filename))
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
            Language::Json | Language::Jsonc => self.json.capabilities(),
            Language::Unknown => self.unknown.capabilities(),
        }
    }
}

/// Checks whether a diagnostic coming from the analyzer is an [error](Severity::Error)
///
/// The function checks the diagnostic against the current configured rules.
pub(crate) fn is_diagnostic_error(
    diagnostic: &'_ AnalyzerDiagnostic,
    rules: Option<&'_ Rules>,
) -> bool {
    let severity = diagnostic
        .category()
        .filter(|category| category.name().starts_with("lint/"))
        .map(|category| {
            rules
                .and_then(|rules| rules.get_severity_from_code(category))
                .unwrap_or(Severity::Warning)
        })
        .unwrap_or_else(|| diagnostic.severity());

    severity >= Severity::Error
}
