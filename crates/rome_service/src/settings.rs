use std::sync::{RwLock, RwLockReadGuard};

use rome_formatter::{IndentStyle, LineWidth};
use rome_fs::RomePath;
use rome_js_syntax::JsLanguage;

/// Global settings for the entire workspace
#[derive(Debug, Default)]
pub struct WorkspaceSettings {
    /// Formatter settings applied to all files in the workspaces
    pub format: FormatSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: LinterSettings,
    /// Language specific settings
    pub languages: LanguagesSettings,
}

/// Formatter settings for the entire workspace
#[derive(Debug)]
pub struct FormatSettings {
    /// Enabled by default
    pub enabled: bool,
    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: bool,
    pub indent_style: Option<IndentStyle>,
    pub line_width: Option<LineWidth>,
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_style: Some(IndentStyle::default()),
            line_width: Some(LineWidth::default()),
        }
    }
}

/// Formatter settings for the entire workspace
#[derive(Debug)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: bool,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Static map of language names to language-specific settings
#[derive(Debug, Default)]
pub struct LanguagesSettings {
    pub javascript: LanguageSettings<JsLanguage>,
}

pub trait Language: rome_rowan::Language {
    /// Formatter settings type for this language
    type FormatSettings: Default;
    /// Fully resolved formatter options type for this language
    type FormatContext: rome_formatter::FormatContext;

    /// Read the settings type for this language from the [LanguagesSettings] map
    fn lookup_settings(languages: &LanguagesSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_context(
        global: &FormatSettings,
        language: &Self::FormatSettings,
        editor: IndentStyle,
        path: &RomePath,
    ) -> Self::FormatContext;
}

#[derive(Debug, Default)]
pub struct LanguageSettings<L: Language> {
    /// Formatter settings for this language
    pub format: L::FormatSettings,
}

/// Handle object holding a temporary lock on the workspace settings until
/// the deferred language-specific options resolution is called
pub(crate) struct SettingsHandle<'a, E> {
    inner: RwLockReadGuard<'a, WorkspaceSettings>,
    /// Additional per-request state injected by the editor
    editor: E,
}

impl<'a, E> SettingsHandle<'a, E> {
    pub(crate) fn new(settings: &'a RwLock<WorkspaceSettings>, editor: E) -> Self {
        Self {
            inner: settings.read().unwrap(),
            editor,
        }
    }
}

impl<'a, E> AsRef<WorkspaceSettings> for SettingsHandle<'a, E> {
    fn as_ref(&self) -> &WorkspaceSettings {
        &*self.inner
    }
}

impl<'a> SettingsHandle<'a, IndentStyle> {
    /// Resolve the formatting context for the given language
    pub(crate) fn format_context<L>(self, path: &RomePath) -> L::FormatContext
    where
        L: Language,
    {
        L::resolve_format_context(
            &self.inner.format,
            &L::lookup_settings(&self.inner.languages).format,
            self.editor,
            path,
        )
    }
}
