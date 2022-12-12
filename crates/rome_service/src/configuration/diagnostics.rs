use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::location::AsSpan;
use rome_diagnostics::{
    Advices, Diagnostic, LineIndexBuf, LogCategory, MessageAndDescription, Visit,
};
use rome_rowan::{SyntaxError, TextRange, TextSize};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Series of errors that can be thrown while computing the configuration
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ConfigurationErrorKind {
    /// Thrown when the program can't serialize the configuration, while saving it
    SerializationError,

    /// Thrown when trying to **create** a new configuration file, but it exists already
    ConfigAlreadyExists,

    /// Error thrown when de-serialising the configuration from file, the issues can be many:
    /// - syntax error
    /// - incorrect fields
    /// - incorrect values
    DeserializationError { text_range: Option<TextRange> },

    /// Thrown when an unknown rule is found
    UnknownRule,

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern,

    /// Error emitted when some content in not correctly parsed/validated by our internal infrastructure
    UnexpectedContent,

    /// A syntax error, an error forwarded from [SyntaxError]
    SyntaxError,
}

#[derive(Diagnostic, Serialize, Deserialize)]
#[diagnostic(category = "configuration", severity = Error)]
pub struct ConfigurationDiagnostic {
    /// The message of the diagnostic
    #[message]
    message: MessageAndDescription,

    /// What kind of error occurred
    kind: ConfigurationErrorKind,

    /// Where the error occurred
    #[location(span)]
    span: Option<TextRange>,

    /// Additional advice
    #[advice]
    advices: ConfigurationAdvices,
}

impl From<SyntaxError> for ConfigurationDiagnostic {
    fn from(_: SyntaxError) -> Self {
        ConfigurationDiagnostic::new_syntax_error()
    }
}

impl ConfigurationDiagnostic {
    fn new(message: impl Display, kind: ConfigurationErrorKind) -> Self {
        Self {
            message: MessageAndDescription::from(markup! { {{message}} }.to_owned()),
            span: None,
            kind,
            advices: ConfigurationAdvices::default(),
        }
    }

    pub(crate) fn new_unknown_rule(message: impl Display) -> Self {
        Self::new(
            markup! {
              "invalid rule name `"{{message}}"`"
            },
            ConfigurationErrorKind::UnknownRule,
        )
    }

    pub(crate) fn new_deserialization_error(message: impl Display) -> Self {
        Self::new(
            markup! {
                "Rome couldn't load the configuration file, here's why: \n"{{message}}
            },
            ConfigurationErrorKind::DeserializationError { text_range: None },
        )
    }

    pub(crate) fn new_unknown_member(key_name: impl Display) -> Self {
        Self::new(
            markup!("Found an extraneous key "<Emphasis>{{ key_name }}</Emphasis> ),
            ConfigurationErrorKind::DeserializationError { text_range: None },
        )
    }

    pub(crate) fn new_unknown_variant(variant_name: impl Display) -> Self {
        Self::new(
            markup!("Found an extraneous variant "<Emphasis>{{ variant_name }}</Emphasis> ),
            ConfigurationErrorKind::DeserializationError { text_range: None },
        )
    }

    pub(crate) fn new_serialization_error() -> Self {
        Self::new(
            "Failed to serialize",
            ConfigurationErrorKind::SerializationError,
        )
    }

    pub(crate) fn new_incorrect_type_for_value(
        key_name: impl Display,
        expected_type: impl Display,
    ) -> Self {
        Self::new(
            markup! {
                "The value of key "<Emphasis>{{key_name}}</Emphasis>" is incorrect. Expected "<Emphasis>{{expected_type}}</Emphasis>
            },
            ConfigurationErrorKind::DeserializationError { text_range: None },
        )
    }

    pub(crate) fn new_incorrect_type(expected_type: impl Display) -> Self {
        Self::new(
            markup! {
                "Incorrect type, expected a "<Emphasis>{{expected_type}}</Emphasis>
            },
            ConfigurationErrorKind::DeserializationError { text_range: None },
        )
    }

    pub(crate) fn new_invalid_ignore_pattern(pattern: impl Display, reason: impl Display) -> Self {
        Self::new(
            markup! { "Couldn't parse the pattern "<Emphasis>{{pattern}}</Emphasis>", reason: "<Emphasis>{{reason}}</Emphasis>"" },
            ConfigurationErrorKind::InvalidIgnorePattern,
        )
    }

    pub(crate) fn new_already_exists() -> Self {
        Self::new(
            "It seems that a configuration file already exists",
            ConfigurationErrorKind::ConfigAlreadyExists,
        )
    }

    pub(crate) fn unexpected(span: impl AsSpan) -> Self {
        Self::new(
            "Unexpected content inside the configuration file",
            ConfigurationErrorKind::UnexpectedContent,
        )
        .with_span(span)
    }

    pub(crate) fn new_syntax_error() -> Self {
        Self::new("Syntax error", ConfigurationErrorKind::SyntaxError)
    }

    /// It adds a span to the diagnostic, useful when the position of the error is known
    pub(crate) fn with_span(mut self, span: impl AsSpan) -> Self {
        self.span = span.as_span();
        self
    }

    /// It adds a suggestion with a list of known keys when an invalid key is found
    pub(crate) fn with_known_keys(
        mut self,
        message: impl Display,
        list: &[&(impl Display + ?Sized)],
    ) -> Self {
        self.advices.known_keys = Some((
            markup! { {message} }.to_owned(),
            list.iter()
                .map(|msg| markup! { {msg} }.to_owned())
                .collect(),
        ));
        self
    }
}

impl Debug for ConfigurationDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ConfigurationErrorKind::SerializationError => std::fmt::Display::fmt(self, f),
            ConfigurationErrorKind::DeserializationError { .. } => std::fmt::Display::fmt(self, f),
            ConfigurationErrorKind::ConfigAlreadyExists => std::fmt::Display::fmt(self, f),
            ConfigurationErrorKind::UnknownRule => std::fmt::Display::fmt(self, f),
            ConfigurationErrorKind::InvalidIgnorePattern => std::fmt::Display::fmt(self, f),
            ConfigurationErrorKind::UnexpectedContent => std::fmt::Display::fmt(self, f),
            ConfigurationErrorKind::SyntaxError => std::fmt::Display::fmt(self, f),
        }
    }
}

impl std::fmt::Display for ConfigurationDiagnostic {
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

impl Diagnostic for ConfigurationDiagnostic {
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigurationAdvices {
    known_keys: Option<(MarkupBuf, Vec<MarkupBuf>)>,
}

impl Advices for ConfigurationAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if let Some((message, known_keys)) = self.known_keys.as_ref() {
            visitor.record_log(LogCategory::Info, message)?;
            let list: Vec<_> = known_keys
                .iter()
                .map(|message| message as &dyn Display)
                .collect();
            visitor.record_list(&list)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::configuration::diagnostics::{from_serde_error_to_range, ConfigurationDiagnostic};
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
            ConfigurationDiagnostic::new_already_exists().with_file_path("rome.json"),
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
                ConfigurationDiagnostic::new_invalid_ignore_pattern(
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
                ConfigurationDiagnostic::new_deserialization_error(error.to_string())
                    .with_span(from_serde_error_to_range(&error, content))
                    .with_file_path("rome.json")
                    .with_file_source_code(content),
            )
        } else {
            panic!("The JSON should be incorrect")
        }
    }
}
