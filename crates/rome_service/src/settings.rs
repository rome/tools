use crate::{
    configuration::FilesConfiguration, Configuration, MatchOptions, Matcher, RomeError, Rules,
};
use indexmap::IndexSet;
use rome_diagnostics::v2::Category;
use rome_formatter::{IndentStyle, LineWidth};
use rome_fs::RomePath;
use rome_js_syntax::JsLanguage;
use std::{
    num::NonZeroU64,
    sync::{RwLock, RwLockReadGuard},
};

/// Global settings for the entire workspace
#[derive(Debug, Default)]
pub struct WorkspaceSettings {
    /// Formatter settings applied to all files in the workspaces
    pub formatter: FormatSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: LinterSettings,
    /// Language specific settings
    pub languages: LanguagesSettings,
    /// Filesystem settings for the workspace
    pub files: FilesSettings,
}

impl WorkspaceSettings {
    /// Retrieves the settings of the formatter
    pub fn formatter(&self) -> &FormatSettings {
        &self.formatter
    }

    /// Retrieves the settings of the linter
    pub fn linter(&self) -> &LinterSettings {
        &self.linter
    }

    /// The (configuration)[Configuration] is merged into the workspace
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn merge_with_configuration(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), RomeError> {
        // formatter part
        if let Some(formatter) = configuration.formatter {
            self.formatter = FormatSettings::try_from(formatter)?;
        }
        let formatter = configuration
            .javascript
            .as_ref()
            .and_then(|j| j.formatter.as_ref());
        if let Some(formatter) = formatter {
            self.languages.javascript.format.quote_style = Some(formatter.quote_style);
            self.languages.javascript.format.quote_properties = Some(formatter.quote_properties);
            self.languages.javascript.format.trailing_comma = Some(formatter.trailing_comma);
        }

        // linter part
        if let Some(linter) = configuration.linter {
            self.linter = LinterSettings::try_from(linter)?;
        }

        let globals = configuration.javascript.and_then(|j| j.globals);
        self.languages.javascript.globals = globals;

        // Filesystem settings
        if let Some(files) = configuration.files {
            self.files = FilesSettings::try_from(files)?;
        }

        Ok(())
    }

    /// It retrieves the severity based on the `code` of the rule and the current configuration.
    ///
    /// The code of the has the following pattern: `{group}/{rule_name}`.
    ///
    /// It returns [None] if the `code` doesn't match any rule.
    pub fn get_severity_from_rule_code(
        &self,
        code: &Category,
    ) -> Option<rome_diagnostics::v2::Severity> {
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
pub struct FormatSettings {
    /// Enabled by default
    pub enabled: bool,
    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: bool,
    pub indent_style: Option<IndentStyle>,
    pub line_width: Option<LineWidth>,
    /// List of paths/files to matcher
    pub ignored_files: Matcher,
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_style: Some(IndentStyle::default()),
            line_width: Some(LineWidth::default()),
            ignored_files: Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            }),
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of rules
    pub rules: Option<Rules>,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
            ignored_files: Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            }),
        }
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

    type LinterSettings: Default;

    /// Fully resolved formatter options type for this language
    type FormatOptions: rome_formatter::FormatOptions;

    /// Read the settings type for this language from the [LanguagesSettings] map
    fn lookup_settings(languages: &LanguagesSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: &FormatSettings,
        language: &Self::FormatSettings,
        path: &RomePath,
    ) -> Self::FormatOptions;
}

#[derive(Debug, Default)]
pub struct LanguageSettings<L: Language> {
    /// Formatter settings for this language
    pub format: L::FormatSettings,

    /// Linter settings for this language
    pub linter: L::LinterSettings,

    /// Globals variables/bindings that can be found in a file
    pub globals: Option<IndexSet<String>>,
}

/// Filesystem settings for the entire workspace
#[derive(Debug)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: NonZeroU64,
}

/// Limit the size of files to 1.0 MiB by default
const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            max_size: DEFAULT_FILE_SIZE_LIMIT,
        }
    }
}

impl TryFrom<FilesConfiguration> for FilesSettings {
    type Error = RomeError;

    fn try_from(config: FilesConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            max_size: config.max_size.unwrap_or(DEFAULT_FILE_SIZE_LIMIT),
        })
    }
}

/// Handle object holding a temporary lock on the workspace settings until
/// the deferred language-specific options resolution is called
#[derive(Debug)]
pub struct SettingsHandle<'a> {
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
    pub(crate) fn format_options<L>(self, path: &RomePath) -> L::FormatOptions
    where
        L: Language,
    {
        L::resolve_format_options(
            &self.inner.formatter,
            &L::lookup_settings(&self.inner.languages).format,
            path,
        )
    }
}
