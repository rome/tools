//! This module contains the configuration of `rome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.

use crate::configuration::formatter::FormatterConfiguration;
use crate::configuration::javascript::JavascriptConfiguration;
use crate::configuration::linter::LinterConfiguration;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};

mod formatter;
mod javascript;
mod linter;

/// The configuration that is contained inside the file `rome.json`
#[derive(Default, Debug, Eq, PartialEq, Deserialize)]
#[serde(default)]
pub struct Configuration {
    /// One root file should exist. Useful when `extends` comes into play.
    ///
    /// If `true`, this file should be the master configuration.
    pub root: bool,

    /// The configuration of the formatter
    pub formatter: FormatterConfiguration,

    /// The configuration for the linter
    pub linter: LinterConfiguration,

    /// Specific configuration for the JavaScript language
    pub javascript: JavascriptConfiguration,
}

impl Configuration {
    pub fn is_formatter_disabled(&self) -> bool {
        !self.formatter.enabled
    }

    pub fn is_linter_disabled(&self) -> bool {
        !self.linter.enabled
    }
}

/// Series of errors that can be thrown while computing the configuration
pub enum ConfigurationError {
    /// Thrown when the main configuration file doesn't have
    NotRoot,
}

impl Debug for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::NotRoot => std::fmt::Display::fmt(self, f),
        }
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::NotRoot => {
                write!(
                f,
                "the main configuration file, rome.json, must have the field 'root' set to `true`"
            )
            }
        }
    }
}
