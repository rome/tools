use crate::{Configuration, Rules};
use indexmap::IndexSet;
use rome_console::codespan::Severity;
use rome_formatter::{IndentStyle, LineWidth};
use rome_fs::RomePath;
use rome_js_syntax::JsLanguage;
use std::sync::{RwLock, RwLockReadGuard};

/// Global settings for the entire workspace
#[derive(Debug, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct WorkspaceSettings {
    /// Formatter settings applied to all files in the workspaces
    #[cfg_attr(feature = "serde_workspace", serde(default))]
    pub format: FormatSettings,
    /// Linter settings applied to all files in the workspace
    #[cfg_attr(feature = "serde_workspace", serde(default))]
    pub linter: LinterSettings,
    /// Language specific settings
    #[cfg_attr(feature = "serde_workspace", serde(default))]
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

    /// It retrieves the severity based on the `code` of the rule and the current configuration.
    ///
    /// The code of the has the following pattern: `{group}/{rule_name}`.
    ///
    /// It returns [None] if the `code` doesn't match any rule.
    pub fn get_severity_from_rule_code(&self, code: &str) -> Option<Severity> {
        let rules = self.linter.rules.as_ref();
        if let Some(rules) = rules {
            rules.get_severity_from_code(code)
        } else {
            None
        }
    }
}

/// Formatter settings for the entire workspace
#[derive(Debug)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
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
#[derive(Debug)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
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
#[derive(Debug, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct LanguagesSettings {
    #[cfg_attr(feature = "serde_workspace", serde(default))]
    pub javascript: LanguageSettings<JsLanguage>,
}

pub trait Language: rome_rowan::Language {
    /// Formatter settings type for this language
    #[cfg(not(feature = "serde_workspace"))]
    type FormatSettings: Default;
    /// Formatter settings type for this language
    #[cfg(feature = "serde_workspace")]
    type FormatSettings: Default
        + serde::Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema;

    /// Linter settings type for this language
    #[cfg(not(feature = "serde_workspace"))]
    type LinterSettings: Default;
    /// Linter settings type for this language
    #[cfg(feature = "serde_workspace")]
    type LinterSettings: Default
        + serde::Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema;

    /// Fully resolved formatter options type for this language
    type FormatContext: rome_formatter::FormatContext;

    /// Read the settings type for this language from the [LanguagesSettings] map
    fn lookup_settings(languages: &LanguagesSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_context(
        global: &FormatSettings,
        language: &Self::FormatSettings,
        path: &RomePath,
    ) -> Self::FormatContext;
}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct LanguageSettings<L: Language> {
    /// Formatter settings for this language
    #[cfg_attr(feature = "serde_workspace", serde(default))]
    pub format: L::FormatSettings,

    /// Linter settings for this language
    #[cfg_attr(feature = "serde_workspace", serde(default))]
    pub linter: L::LinterSettings,

    /// Globals variables/bindings that can be found in a file
    #[cfg_attr(
        feature = "serde_workspace",
        serde(
            default,
            deserialize_with = "crate::configuration::deserialize_globals",
            serialize_with = "crate::configuration::serialize_globals"
        )
    )]
    pub globals: IndexSet<String>,
}

/// Handle object holding a temporary lock on the workspace settings until
/// the deferred language-specific options resolution is called
#[derive(Debug)]
pub(crate) struct SettingsHandle<'a> {
    inner: RwLockReadGuard<'a, WorkspaceSettings>,
}

impl<'a> SettingsHandle<'a> {
    pub(crate) fn new(settings: &'a RwLock<WorkspaceSettings>) -> Self {
        Self {
            inner: settings.read().unwrap(),
        }
    }
}

impl<'a> AsRef<WorkspaceSettings> for SettingsHandle<'a> {
    fn as_ref(&self) -> &WorkspaceSettings {
        &*self.inner
    }
}

impl<'a> SettingsHandle<'a> {
    /// Resolve the formatting context for the given language
    pub(crate) fn format_context<L>(self, path: &RomePath) -> L::FormatContext
    where
        L: Language,
    {
        L::resolve_format_context(
            &self.inner.format,
            &L::lookup_settings(&self.inner.languages).format,
            path,
        )
    }
}
