use rome_diagnostics::{category, Category, Diagnostic, LineIndexBuf, Location};
use rome_rowan::{TextRange, TextSize};
use std::fmt::{Debug, Display, Formatter};

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
    DeserializationError {
        message: String,
        text_range: Option<TextRange>,
        input: String,
    },

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern(String, String),
}

impl Debug for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SerializationError => std::fmt::Display::fmt(self, f),
            ConfigurationError::DeserializationError { .. } => std::fmt::Display::fmt(self, f),
            ConfigurationError::ConfigAlreadyExists => std::fmt::Display::fmt(self, f),
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
                    "Couldn't save the configuration on disk, probably because of some error inside the content of the file"
                )
            }
            ConfigurationError::DeserializationError { message, .. } => {
                write!(
                    f,
                    "Rome couldn't load the configuration file, here's why: \n{}",
                    message
                )
            }
            ConfigurationError::ConfigAlreadyExists => {
                write!(f, "It seems that a configuration file already exists")
            }

            ConfigurationError::InvalidIgnorePattern(pattern, reason) => {
                write!(f, "Couldn't parse the pattern {pattern}, reason: {reason}")
            }
        }
    }
}

impl Diagnostic for ConfigurationError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("configuration"))
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }

    fn message(&self, f: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            ConfigurationError::SerializationError => {
                write!(
                    f,
                    "Couldn't save the configuration on disk, probably because of some error inside the content of the file"
                )
            }
            ConfigurationError::DeserializationError { message, .. } => {
                write!(
                    f,
                    "Rome couldn't load the configuration file, here's why: \n{}",
                    message
                )
            }
            ConfigurationError::ConfigAlreadyExists => {
                write!(f, "It seems that a configuration file already exists")
            }

            ConfigurationError::InvalidIgnorePattern(pattern, reason) => {
                write!(f, "Couldn't parse the pattern {pattern}, reason: {reason}")
            }
        }
    }

    fn location(&self) -> Location<'_> {
        if let Self::DeserializationError { text_range, .. } = self {
            Location::builder().span(text_range).build()
        } else {
            Location::builder().build()
        }
    }
}

pub(crate) fn from_serde_error_to_range(
    error: &serde_json::Error,
    input: &str,
) -> Option<TextRange> {
    let line_starts = LineIndexBuf::from_source_text(input);
    let line = error.line();
    line.checked_sub(1).and_then(|line_index| {
        let line_start = line_starts.get(line_index)?;
        let column_index = error.column().checked_sub(1)?;
        let column_offset = TextSize::try_from(column_index).ok()?;

        let span_start = line_start + column_offset;
        Some(TextRange::at(span_start, TextSize::from(0)))
    })
}

#[cfg(test)]
mod test {
    use crate::configuration::diagnostics::{from_serde_error_to_range, ConfigurationError};
    use crate::{Configuration, MatchOptions, Matcher};
    use rome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error};

    fn snap_diagnostic(test_name: &str, diagnostic: Error) {
        let content = print_diagnostic_to_string(&diagnostic);

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);

        });
    }

    #[test]
    fn config_already_exists() {
        snap_diagnostic(
            "config_already_exists",
            ConfigurationError::ConfigAlreadyExists.with_file_path("rome.json"),
        )
    }

    #[test]
    fn incorrect_pattern() {
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });

        let pattern = "*******";
        if let Err(error) = matcher.add_pattern(pattern) {
            snap_diagnostic(
                "incorrect_pattern",
                ConfigurationError::InvalidIgnorePattern(
                    pattern.to_string(),
                    error.msg.to_string(),
                )
                .with_file_path("rome.json"),
            )
        } else {
            panic!("Tha pattern should fail")
        }
    }

    #[test]
    fn deserialization_error() {
        let content = "{ \n\n\"formatter\" }";
        let result = serde_json::from_str::<Configuration>(content);
        if let Err(error) = result {
            snap_diagnostic(
                "deserialization_error",
                ConfigurationError::DeserializationError {
                    text_range: from_serde_error_to_range(&error, content),
                    input: content.to_string(),
                    message: error.to_string(),
                }
                .with_file_path("rome.json")
                .with_file_source_code(content),
            )
        } else {
            panic!("The JSON should be incorrect")
        }
    }
}
