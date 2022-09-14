//! This module contains the configuration of `rome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.

use crate::{DynRef, RomeError};
use rome_fs::{FileSystem, OpenOptions};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind;
use std::path::PathBuf;
use tracing::{error, info};

mod formatter;
mod javascript;
pub mod linter;

pub use formatter::{FormatterConfiguration, PlainIndentStyle};
pub use javascript::{JavascriptConfiguration, JavascriptFormatter};
pub use linter::{LinterConfiguration, RuleConfiguration, Rules};

/// The configuration that is contained inside the file `rome.json`
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Configuration {
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
            linter: Some(LinterConfiguration {
                enabled: true,
                ..LinterConfiguration::default()
            }),
            formatter: None,
            javascript: None,
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
}

impl Debug for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SerializationError => std::fmt::Display::fmt(self, f),
            ConfigurationError::DeserializationError(_) => std::fmt::Display::fmt(self, f),
            ConfigurationError::ConfigAlreadyExists => std::fmt::Display::fmt(self, f),
            ConfigurationError::UnknownRule(_) => std::fmt::Display::fmt(self, f),
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
    configuration: Configuration,
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

    let contents = serde_json::to_string_pretty(&configuration)
        .map_err(|_| RomeError::Configuration(ConfigurationError::SerializationError))?;

    config_file
        .set_content(contents.as_bytes())
        .map_err(|_| RomeError::CantReadFile(path))?;

    Ok(())
}
