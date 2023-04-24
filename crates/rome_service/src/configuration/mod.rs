//! This module contains the configuration of `rome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.

use crate::{DynRef, WorkspaceError};
use bpaf::Bpaf;
use rome_fs::{FileSystem, OpenOptions};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

pub mod diagnostics;
pub mod formatter;
mod generated;
pub mod javascript;
pub mod linter;
pub mod organize_imports;
mod parse;
pub mod string_set;
pub mod vcs;

pub use crate::configuration::diagnostics::ConfigurationDiagnostic;
use crate::configuration::generated::push_to_analyzer_rules;
use crate::configuration::organize_imports::{organize_imports, OrganizeImports};
pub use crate::configuration::string_set::StringSet;
use crate::configuration::vcs::{vcs_configuration, VcsConfiguration};
use crate::settings::{LanguagesSettings, LinterSettings};
pub use formatter::{formatter_configuration, FormatterConfiguration, PlainIndentStyle};
pub use javascript::{javascript_configuration, JavascriptConfiguration, JavascriptFormatter};
pub use linter::{linter_configuration, LinterConfiguration, RuleConfiguration, Rules};
use rome_analyze::{AnalyzerConfiguration, AnalyzerRules};
use rome_deserialize::json::deserialize_from_json_str;
use rome_deserialize::Deserialized;
use rome_js_analyze::metadata;
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_parser::parse_json;

/// The configuration that is contained inside the file `rome.json`
#[derive(Debug, Deserialize, Serialize, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RomeConfiguration {
    /// A field for the [JSON schema](https://json-schema.org/) specification
    #[serde(rename(serialize = "$schema", deserialize = "$schema"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub schema: Option<String>,

    /// The configuration of the filesystem
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional, hide_usage)]
    pub vcs_configuration: Option<VcsConfiguration>,

    /// The configuration of the filesystem
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional, hide_usage)]
    pub files_configuration: Option<FilesConfiguration>,

    /// The configuration of the formatter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional)]
    pub formatter_configuration: Option<FormatterConfiguration>,

    /// The configuration of the import sorting
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional)]
    pub organize_imports: Option<OrganizeImports>,

    /// The configuration for the linter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional)]
    pub linter_configuration: Option<LinterConfiguration>,

    /// Specific configuration for the JavaScript language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional)]
    pub javascript_configuration: Option<JavascriptConfiguration>,
}

impl Default for RomeConfiguration {
    fn default() -> Self {
        Self {
            files_configuration: None,
            linter_configuration: Some(LinterConfiguration {
                enabled: true,
                ..LinterConfiguration::default()
            }),
            organize_imports: Some(OrganizeImports::default()),
            formatter_configuration: None,
            javascript_configuration: None,
            schema: None,
            vcs_configuration: None,
        }
    }
}

impl RomeConfiguration {
    const KNOWN_KEYS: &'static [&'static str] = &[
        "vcs",
        "files",
        "linter",
        "formatter",
        "javascript",
        "$schema",
        "organizeImports",
    ];
}

impl RomeConfiguration {
    pub fn is_formatter_disabled(&self) -> bool {
        self.formatter_configuration
            .as_ref()
            .map(|f| !f.enabled)
            .unwrap_or(false)
    }

    pub fn is_linter_disabled(&self) -> bool {
        self.linter_configuration
            .as_ref()
            .map(|f| !f.enabled)
            .unwrap_or(false)
    }

    pub fn is_organize_imports_disabled(&self) -> bool {
        self.organize_imports
            .as_ref()
            .map(|f| !f.enabled)
            .unwrap_or(false)
    }

    pub fn is_vcs_disabled(&self) -> bool {
        self.vcs_configuration
            .as_ref()
            .map(|f| matches!(f.enabled, Some(false)))
            .unwrap_or(true)
    }
}

/// The configuration of the filesystem
#[derive(Default, Debug, Deserialize, Serialize, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reason. Defaults to 1 MiB
    #[bpaf(long("files-max-size"), argument("NUMBER"))]
    pub max_size: Option<NonZeroU64>,

    /// A list of Unix shell style patterns. Rome tools will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,
}

impl FilesConfiguration {
    const KNOWN_KEYS: &'static [&'static str] = &["maxSize", "ignore"];
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [Deserialized]: result of the deserialization of the configuration.
/// - [RomeConfiguration]: the type needed to [Deserialized] to infer the return type.
/// - [PathBuf]: the path of where the first `rome.json` path was found
type LoadConfig = Result<Option<(Deserialized<RomeConfiguration>, PathBuf)>, WorkspaceError>;

#[derive(Debug, Default, PartialEq)]
pub enum ConfigurationBasePath {
    /// The default mode, not having a configuration file is not an error.
    #[default]
    None,
    /// The base path provided by the LSP, not having a configuration file is not an error.
    Lsp(PathBuf),
    /// The base path provided by the user, not having a configuration file is an error.
    /// Throws any kind of I/O errors.
    FromUser(PathBuf),
}

impl ConfigurationBasePath {
    const fn is_from_user(&self) -> bool {
        matches!(self, ConfigurationBasePath::FromUser(_))
    }
}

/// Load the configuration from the file system.
///
/// The configuration file will be read from the `file_system`. A [base path](ConfigurationBasePath) should be provided.
///
/// The function will try to traverse upwards the file system until if finds a `rome.json` file, or there
/// aren't directories anymore.
///
/// If a the configuration base path was provided by the user, the function will error. If not, Rome will use
/// its defaults.
pub fn load_config(
    file_system: &DynRef<dyn FileSystem>,
    base_path: ConfigurationBasePath,
) -> LoadConfig {
    let config_name = file_system.config_name();
    let working_directory = file_system.working_directory();
    let configuration_directory = match base_path {
        ConfigurationBasePath::Lsp(ref path) | ConfigurationBasePath::FromUser(ref path) => {
            path.clone()
        }
        _ => match working_directory {
            Some(wd) => wd,
            None => PathBuf::new(),
        },
    };
    let configuration_file_path = configuration_directory.join(config_name);
    let should_error = base_path.is_from_user();

    let result = file_system.auto_search(configuration_directory, config_name, should_error)?;

    if let Some((buffer, configuration_path)) = result {
        let deserialized = deserialize_from_json_str::<RomeConfiguration>(&buffer)
            .with_file_path(&configuration_file_path.display().to_string());
        Ok(Some((deserialized, configuration_path)))
    } else {
        Ok(None)
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
    mut configuration: RomeConfiguration,
) -> Result<(), WorkspaceError> {
    let path = PathBuf::from(fs.config_name());

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            WorkspaceError::Configuration(ConfigurationDiagnostic::new_already_exists())
        } else {
            WorkspaceError::cant_read_file(format!("{}", path.display()))
        }
    })?;

    // we now check if rome is installed inside `node_modules` and if so, we
    let schema_path = Path::new("./node_modules/rome/configuration_schema.json");
    let options = OpenOptions::default().read(true);
    if fs.open_with_options(schema_path, options).is_ok() {
        configuration.schema = schema_path.to_str().map(String::from);
    }

    let contents = serde_json::to_string_pretty(&configuration).map_err(|_| {
        WorkspaceError::Configuration(ConfigurationDiagnostic::new_serialization_error())
    })?;

    let parsed = parse_json(&contents);
    let formatted =
        rome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())?
            .print()
            .expect("valid format document");

    config_file
        .set_content(formatted.as_code().as_bytes())
        .map_err(|_| WorkspaceError::cant_read_file(format!("{}", path.display())))?;

    Ok(())
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
        push_to_analyzer_rules(rules, metadata(), &mut analyzer_rules);
    }

    AnalyzerConfiguration {
        globals,
        rules: analyzer_rules,
    }
}
