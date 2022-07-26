use crate::{database::Formatter, Configuration, Rules};
use indexmap::IndexSet;
use rome_formatter::{IndentStyle, LineWidth};
use rome_fs::RomePath;
use rome_js_syntax::JsLanguage;

/// Global settings for the entire workspace
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct WorkspaceSettings {
    /// Formatter settings applied to all files in the workspaces
    pub format: FormatSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: LinterSettings,
    /// Language specific settings
    pub languages: LanguagesSettings,
}

impl WorkspaceSettings {
    /// The (configuration)[Configuration] is merged into the workspace
    pub fn merge_with_configuration(&mut self, configuration: Configuration) {
        // formatter part
        if let Some(formatter) = configuration.formatter {
            self.format = FormatSettings::from(formatter);
        }
        let formatter = configuration
            .javascript
            .as_ref()
            .and_then(|j| j.formatter.as_ref());
        if let Some(formatter) = formatter {
            self.languages.javascript.format.quote_style = Some(formatter.quote_style);
        }

        // linter part
        if let Some(linter) = configuration.linter {
            self.linter = LinterSettings::from(linter)
        }

        let globals = configuration.javascript.map(|j| j.globals);
        if let Some(globals) = globals {
            self.languages.javascript.globals = globals;
        }
    }
}

/// Formatter settings for the entire workspace
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
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

/// Linter settings for the entire workspace
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of rules
    pub rules: Option<Rules>,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
        }
    }
}

/// Static map of language names to language-specific settings
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LanguagesSettings {
    pub javascript: LanguageSettings<JsLanguage>,
}

pub trait Language: rome_rowan::Language {
    /// Formatter settings type for this language
    type FormatSettings: Default;

    /// Linter settings type for this language
    type LinterSettings: Default;

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

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LanguageSettings<L: Language> {
    /// Formatter settings for this language
    pub format: L::FormatSettings,

    /// Linter settings for this language
    pub linter: L::LinterSettings,

    /// Globals variables/bindings that can be found in a file
    #[cfg_attr(
        feature = "serde_workspace",
        serde(
            deserialize_with = "crate::configuration::deserialize_globals",
            serialize_with = "crate::configuration::serialize_globals"
        )
    )]
    pub globals: IndexSet<String>,
}

/// Handle object holding a temporary lock on the workspace settings until
/// the deferred language-specific options resolution is called
pub(crate) struct SettingsHandle<'a, E> {
    db: &'a dyn Formatter,
    /// Additional per-request state injected by the editor
    editor: E,
}

impl<'a, E> SettingsHandle<'a, E> {
    pub(crate) fn new(db: &'a dyn Formatter, editor: E) -> Self {
        Self { db, editor }
    }

    pub(crate) fn settings(&self) -> WorkspaceSettings {
        self.db.settings(())
    }
}

impl<'a> SettingsHandle<'a, IndentStyle> {
    /// Resolve the formatting context for the given language
    pub(crate) fn format_context<L>(self, path: &RomePath) -> L::FormatContext
    where
        L: Language,
    {
        let settings = self.settings();
        L::resolve_format_context(
            &settings.format,
            &L::lookup_settings(&settings.languages).format,
            self.editor,
            path,
        )
    }
}
