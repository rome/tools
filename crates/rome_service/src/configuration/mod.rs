//! This module contains the configuration of `rome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.

use crate::configuration::formatter::FormatterConfiguration;
use crate::configuration::javascript::JavascriptConfiguration;
use serde::Deserialize;

mod formatter;
mod javascript;

/// The configuration that is contained inside the file `rome.json`
#[derive(Default, Debug, Eq, PartialEq, Deserialize)]
#[serde(default)]
pub struct Configuration {
    /// The configuration of the formatter
    pub formatter: FormatterConfiguration,

    /// Specific configuration for the JavaScript language
    pub javascript: JavascriptConfiguration,
}
