//! This module contains the configuration of `rome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.

use crate::{DynRef, RomeError};
use indexmap::{IndexMap, IndexSet};
use rome_fs::{FileSystem, OpenOptions};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind;
use std::marker::PhantomData;
use std::num::NonZeroU64;
use std::path::PathBuf;
use tracing::{error, info};

mod formatter;
mod javascript;
pub mod linter;
use crate::settings::{LanguagesSettings, LinterSettings};
pub use formatter::{FormatterConfiguration, PlainIndentStyle};
pub use javascript::{JavascriptConfiguration, JavascriptFormatter};
pub use linter::{LinterConfiguration, RuleConfiguration, Rules};
use rome_analyze::{AnalyzerConfiguration, AnalyzerRules, MetadataRegistry};
use rome_js_analyze::metadata;

/// The configuration that is contained inside the file `rome.json`
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    /// A field for the [JSON schema](https://json-schema.org/) specification
    #[serde(rename(serialize = "$schema", deserialize = "$schema"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// The configuration of the filesystem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<FilesConfiguration>,

    /// The configuration of the formatter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<FormatterConfiguration>,

    /// The configuration for the linter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<LinterConfiguration>,

    /// Specific configuration for the JavaScript language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript: Option<JavascriptConfiguration>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            files: None,
            linter: Some(LinterConfiguration {
                enabled: true,
                ..LinterConfiguration::default()
            }),
            formatter: None,
            javascript: None,
            schema: None,
        }
    }
}

impl Configuration {
    pub fn is_formatter_disabled(&self) -> bool {
        self.formatter.as_ref().map(|f| !f.enabled).unwrap_or(false)
    }

    pub fn is_linter_disabled(&self) -> bool {
        self.linter.as_ref().map(|f| !f.enabled).unwrap_or(false)
    }
}

/// The configuration of the filesystem
#[derive(Default, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reason. Defaults to 1 MiB
    pub max_size: Option<NonZeroU64>,

    /// A list of Unix shell style patterns. Rome tools will ignore files/folders that will
    /// match these patterns.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::deserialize_set_of_strings",
        serialize_with = "crate::serialize_set_of_strings"
    )]
    pub ignore: Option<IndexSet<String>>,
}

/// Series of errors that can be thrown while computing the configuration
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ConfigurationError {
    /// Thrown when the program can't serialize the configuration, while saving it
    SerializationError,

    /// Thrown when trying to **create** a new configuration file, but it exists already
    ConfigAlreadyExists,

    /// Error thrown when de-serialising the configuration from file, the issues can be many:
    /// - syntax error
    /// - incorrect fields
    /// - incorrect values
    DeserializationError(String),

    /// Thrown when an unknown rule is found
    UnknownRule(String),

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern(String, String),
}

impl Debug for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SerializationError => std::fmt::Display::fmt(self, f),
            ConfigurationError::DeserializationError(_) => std::fmt::Display::fmt(self, f),
            ConfigurationError::ConfigAlreadyExists => std::fmt::Display::fmt(self, f),
            ConfigurationError::UnknownRule(_) => std::fmt::Display::fmt(self, f),
            ConfigurationError::InvalidIgnorePattern(_, _) => std::fmt::Display::fmt(self, f),
        }
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SerializationError => {
                write!(
                    f,
                    "couldn't save the configuration on disk, probably because of some error inside the content of the file"
                )
            }
            ConfigurationError::DeserializationError(reason) => {
                write!(
                    f,
                    "Rome couldn't load the configuration file, here's why: \n{}",
                    reason
                )
            }
            ConfigurationError::ConfigAlreadyExists => {
                write!(f, "it seems that a configuration file already exists")
            }

            ConfigurationError::UnknownRule(rule) => {
                write!(f, "invalid rule name `{rule}`")
            }
            ConfigurationError::InvalidIgnorePattern(pattern, reason) => {
                write!(f, "couldn't parse the pattern {pattern}, reason: {reason}")
            }
        }
    }
}

/// This function is responsible to load the rome configuration.
///
/// The `file_system` will read the configuration file. A base path can be passed
pub fn load_config(
    file_system: &DynRef<dyn FileSystem>,
    base_path: Option<PathBuf>,
) -> Result<Option<Configuration>, RomeError> {
    let config_name = file_system.config_name();
    let configuration_path = if let Some(base_path) = base_path {
        base_path.join(config_name)
    } else {
        PathBuf::from(config_name)
    };
    info!(
        "Attempting to load the configuration file at path {:?}",
        configuration_path
    );
    let options = OpenOptions::default().read(true).write(true);
    let file = file_system.open_with_options(&configuration_path, options);
    match file {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)
                .map_err(|_| RomeError::CantReadFile(configuration_path))?;

            let configuration: Configuration = serde_json::from_str(&buffer).map_err(|err| {
                RomeError::Configuration(ConfigurationError::DeserializationError(err.to_string()))
            })?;

            Ok(Some(configuration))
        }
        Err(err) => {
            // We throw an error only when the error is found.
            // In case we don't fine the file, we swallow the error and we continue; not having
            // a file should not be a cause of error (for now)
            if err.kind() != ErrorKind::NotFound {
                return Err(RomeError::CantReadFile(configuration_path));
            }
            error!(
                "Could not find the file configuration at {:?}",
                configuration_path.display()
            );
            error!("Reason: {:?}", err);
            Ok(None)
        }
    }
}

/// Creates a new configuration on file system
///
/// ## Errors
///
/// It fails if:
/// - the configuration file already exists
/// - the program doesn't have the write rights
pub fn create_config(
    fs: &mut DynRef<dyn FileSystem>,
    mut configuration: Configuration,
) -> Result<(), RomeError> {
    let path = PathBuf::from(fs.config_name());

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            RomeError::Configuration(ConfigurationError::ConfigAlreadyExists)
        } else {
            RomeError::CantReadFile(path.clone())
        }
    })?;

    // we now check if rome is installed inside `node_modules` and if so, we
    let schema_path = PathBuf::from("./")
        .join("node_modules")
        .join("rome")
        .join("configuration_schema.json");
    let options = OpenOptions::default().read(true);
    if let Ok(_) = fs.open_with_options(&schema_path, options) {
        configuration.schema = schema_path.to_str().map(String::from);
    }

    let contents = serde_json::to_string_pretty(&configuration)
        .map_err(|_| RomeError::Configuration(ConfigurationError::SerializationError))?;

    config_file
        .set_content(contents.as_bytes())
        .map_err(|_| RomeError::CantReadFile(path))?;

    Ok(())
}

/// Some documentation
pub fn deserialize_set_of_strings<'de, D>(
    deserializer: D,
) -> Result<Option<IndexSet<String>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct IndexVisitor {
        marker: PhantomData<fn() -> Option<IndexSet<String>>>,
    }

    impl IndexVisitor {
        fn new() -> Self {
            IndexVisitor {
                marker: PhantomData,
            }
        }
    }

    impl<'de> Visitor<'de> for IndexVisitor {
        type Value = Option<IndexSet<String>>;

        // Format a message stating what data this Visitor expects to receive.
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expecting a sequence")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut index_set = IndexSet::with_capacity(seq.size_hint().unwrap_or(0));

            while let Some(value) = seq.next_element()? {
                index_set.insert(value);
            }

            Ok(Some(index_set))
        }
    }

    deserializer.deserialize_seq(IndexVisitor::new())
}

pub fn serialize_set_of_strings<S>(
    set_of_strings: &Option<IndexSet<String>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    if let Some(set_of_strings) = set_of_strings {
        let mut sequence = s.serialize_seq(Some(set_of_strings.len()))?;
        let iter = set_of_strings.into_iter();
        for global in iter {
            sequence.serialize_element(global)?;
        }

        sequence.end()
    } else {
        s.serialize_none()
    }
}

/// Converts a [WorkspaceSettings] into a suited [configuration for the analyzer].
///
/// The function needs access to a filter, in order to have an easy access to the [metadata] of the
/// rules.
///
/// The third argument is a closure that accepts a reference to `linter_settings`.
///
/// The closure is responsible to map the globals from the correct
/// location of the settings.
///
/// ## Examples
///
/// ```rust
/// use rome_service::configuration::to_analyzer_configuration;
/// use rome_service::settings::{LanguagesSettings, WorkspaceSettings};
/// let mut settings = WorkspaceSettings::default();
/// settings.languages.javascript.globals = Some(["jQuery".to_string(), "React".to_string()].into());
/// // map globals from JS language
/// let analyzer_configuration =
///     to_analyzer_configuration(&settings.linter, &settings.languages, |settings| {
///         if let Some(globals) = settings.javascript.globals.as_ref() {
///             globals
///                 .iter()
///                 .map(|global| global.to_string())
///                 .collect::<Vec<_>>()
///         } else {
///             vec![]
///         }
///     });
///
///  assert_eq!(
///     analyzer_configuration.globals,
///     vec!["jQuery".to_string(), "React".to_string()]
///  )
/// ```
///
/// [WorkspaceSettings]: crate::settings::WorkspaceSettings
/// [metadata]: rome_analyze::RegistryRuleMetadata
/// [configuration for the analyzer]: AnalyzerConfiguration
pub fn to_analyzer_configuration<ToGlobals>(
    linter_settings: &LinterSettings,
    language_settings: &LanguagesSettings,
    to_globals: ToGlobals,
) -> AnalyzerConfiguration
where
    ToGlobals: FnOnce(&LanguagesSettings) -> Vec<String>,
{
    let globals: Vec<String> = to_globals(language_settings);

    let mut analyzer_rules = AnalyzerRules::default();

    if let Some(rules) = linter_settings.rules.as_ref() {
        if let Some(rules) = rules.correctness.as_ref() {
            push_rules("correctness", metadata(), &mut analyzer_rules, &rules.rules);
        }
        if let Some(rules) = rules.nursery.as_ref() {
            push_rules("nursery", metadata(), &mut analyzer_rules, &rules.rules);
        }
        if let Some(rules) = rules.style.as_ref() {
            push_rules("style", metadata(), &mut analyzer_rules, &rules.rules);
        }
    }

    AnalyzerConfiguration {
        globals,
        rules: analyzer_rules,
    }
}

fn push_rules(
    group_name: &'static str,
    metadata: &MetadataRegistry,
    analyzer_rules: &mut AnalyzerRules,
    rules: &IndexMap<String, RuleConfiguration>,
) {
    for (rule_name, configuration) in rules {
        if let RuleConfiguration::WithOptions(rule_options) = configuration {
            if let Some(options) = &rule_options.options {
                if let Some(rule_key) = metadata.find_rule(group_name, rule_name) {
                    analyzer_rules.push_rule(rule_key, options.clone());
                }
            }
        }
    }
}
