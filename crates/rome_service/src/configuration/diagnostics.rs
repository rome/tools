use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::location::AsSpan;
use rome_diagnostics::{
    Advices, Category, Diagnostic, DiagnosticTags, LineIndexBuf, Location, LogCategory, Severity,
    Visit,
};
use rome_rowan::{SyntaxError, TextRange, TextSize};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Series of errors that can be thrown while computing the configuration
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ConfigurationDiagnostic {
    /// Thrown when the program can't serialize the configuration, while saving it
    SerializationError(SerializationError),

    /// Thrown when trying to **create** a new configuration file, but it exists already
    ConfigAlreadyExists(ConfigAlreadyExists),

    /// Error thrown when de-serialising the configuration from file, the issues can be many:
    /// - syntax error
    /// - incorrect fields
    /// - incorrect values
    Deserialization(Deserialization),

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern(InvalidIgnorePattern),
}

impl From<SyntaxError> for ConfigurationDiagnostic {
    fn from(_: SyntaxError) -> Self {
        ConfigurationDiagnostic::Deserialization(Deserialization {
            deserialization_advice: DeserializationAdvice::default(),
            range: None,
            reason: markup! {"Syntax Error"}.to_owned(),
        })
    }
}

impl ConfigurationDiagnostic {
    pub(crate) fn new_deserialization_error(reason: impl Display, span: impl AsSpan) -> Self {
        Self::Deserialization(Deserialization {
            range: span.as_span(),
            reason: markup! {{reason}}.to_owned(),
            deserialization_advice: DeserializationAdvice::default(),
        })
    }

    pub(crate) fn new_unknown_member(
        key_name: impl Display,
        range: impl AsSpan,
        known_members: &[&str],
    ) -> Self {
        Self::Deserialization(Deserialization {
            range: range.as_span(),
            reason: markup!("Found an extraneous key "<Emphasis>{{ key_name }}</Emphasis> )
                .to_owned(),

            deserialization_advice: DeserializationAdvice {
                known_keys: Some((
                    markup! { "Accepted keys" }.to_owned(),
                    known_members
                        .iter()
                        .map(|message| markup! {{message}}.to_owned())
                        .collect::<Vec<_>>(),
                )),
                ..DeserializationAdvice::default()
            },
        })
    }

    pub(crate) fn new_unknown_variant(
        variant_name: impl Display,
        range: impl AsSpan,
        known_variants: &[&str],
    ) -> Self {
        Self::Deserialization(Deserialization {
            reason: markup!("Found an extraneous variant "<Emphasis>{{ variant_name }}</Emphasis> )
                .to_owned(),
            range: range.as_span(),
            deserialization_advice: DeserializationAdvice {
                known_keys: Some((
                    markup! { "Accepted values" }.to_owned(),
                    known_variants
                        .iter()
                        .map(|message| markup! {{message}}.to_owned())
                        .collect::<Vec<_>>(),
                )),
                ..DeserializationAdvice::default()
            },
        })
    }

    pub(crate) fn new_serialization_error() -> Self {
        Self::SerializationError(SerializationError)
    }

    pub(crate) fn new_incorrect_type_for_value(
        key_name: impl Display,
        expected_type: impl Display,
        range: impl AsSpan,
    ) -> Self {
        Self::Deserialization(Deserialization {
            range: range.as_span(),
            reason: markup! {
                "The value of key "<Emphasis>{{key_name}}</Emphasis>" is incorrect. Expected "<Emphasis>{{expected_type}}</Emphasis>
            }.to_owned(),
            deserialization_advice: DeserializationAdvice::default(),
        })
    }

    pub(crate) fn new_incorrect_type(expected_type: impl Display, range: impl AsSpan) -> Self {
        Self::Deserialization(Deserialization {
            range: range.as_span(),
            reason: markup! {
                "Incorrect type, expected a "<Emphasis>{{expected_type}}</Emphasis>
            }
            .to_owned(),

            deserialization_advice: DeserializationAdvice::default(),
        })
    }

    pub(crate) fn new_invalid_ignore_pattern(pattern: impl Display, reason: impl Display) -> Self {
        Self::Deserialization(Deserialization {
            reason:
                markup! { "Couldn't parse the pattern "<Emphasis>{{pattern}}</Emphasis>", reason: "<Emphasis>{{reason}}</Emphasis>"" }.to_owned()
            ,
            range: None,
            deserialization_advice: DeserializationAdvice::default()
        })
    }

    pub(crate) fn new_already_exists() -> Self {
        Self::Deserialization(Deserialization {
            reason: markup!("It seems that a configuration file already exists").to_owned(),
            range: None,
            deserialization_advice: DeserializationAdvice::default(),
        })
    }

    pub(crate) fn unexpected(span: impl AsSpan) -> Self {
        Self::Deserialization(Deserialization {
            reason: markup!("Unexpected content inside the configuration file").to_owned(),

            range: span.as_span(),
            deserialization_advice: DeserializationAdvice::default(),
        })
    }
    pub(crate) fn new_syntax_error() -> Self {
        Self::from(SyntaxError::MissingRequiredChild)
    }
}

impl Debug for ConfigurationDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for ConfigurationDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)
    }
}

impl Diagnostic for ConfigurationDiagnostic {
    fn severity(&self) -> Severity {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.severity(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.severity(),
            ConfigurationDiagnostic::Deserialization(error) => error.severity(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.severity(),
        }
    }

    fn category(&self) -> Option<&'static Category> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.category(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.category(),
            ConfigurationDiagnostic::Deserialization(error) => error.category(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.category(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.tags(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.tags(),
            ConfigurationDiagnostic::Deserialization(error) => error.tags(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.tags(),
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.location(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.location(),
            ConfigurationDiagnostic::Deserialization(error) => error.location(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.location(),
        }
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.source(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.source(),
            ConfigurationDiagnostic::Deserialization(error) => error.source(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.source(),
        }
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.message(fmt),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.message(fmt),
            ConfigurationDiagnostic::Deserialization(error) => error.message(fmt),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.message(fmt),
        }
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.description(fmt),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.description(fmt),
            ConfigurationDiagnostic::Deserialization(error) => error.description(fmt),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.description(fmt),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.advices(visitor),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.advices(visitor),
            ConfigurationDiagnostic::Deserialization(error) => error.advices(visitor),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.advices(visitor),
        }
    }

    fn verbose_advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::Deserialization(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.verbose_advices(visitor),
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
    hint: Option<MarkupBuf>,
    known_keys: Option<(MarkupBuf, Vec<MarkupBuf>)>,
}

impl Advices for ConfigurationAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if let Some(hint) = self.hint.as_ref() {
            visitor.record_log(LogCategory::Info, hint)?;
        }

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

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "Failed to serialize",
    category = "configuration",
    severity = Error
)]
pub struct SerializationError;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "It seems that a configuration file already exists",
    category = "configuration",
    severity = Error
)]
pub struct ConfigAlreadyExists {}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error
)]
pub struct InvalidIgnorePattern {
    source: String,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error
)]
pub struct Deserialization {
    #[message]
    pub(crate) reason: MarkupBuf,
    #[location(span)]
    pub(crate) range: Option<TextRange>,
    #[advice]
    pub(crate) deserialization_advice: DeserializationAdvice,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeserializationAdvice {
    pub(crate) hint: Option<MarkupBuf>,
    pub(crate) known_keys: Option<(MarkupBuf, Vec<MarkupBuf>)>,
}

impl Advices for DeserializationAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if let Some(hint) = self.hint.as_ref() {
            visitor.record_log(LogCategory::Info, hint)?;
        }

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
    use crate::configuration::diagnostics::{
        from_serde_error_to_range, ConfigurationDiagnostic, Deserialization, DeserializationAdvice,
    };
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
    fn diagnostic_size() {
        assert_eq!(std::mem::size_of::<ConfigurationDiagnostic>(), 112);
        assert_eq!(std::mem::size_of::<Deserialization>(), 112);
        assert_eq!(std::mem::size_of::<DeserializationAdvice>(), 72);
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
                ConfigurationDiagnostic::new_deserialization_error(
                    error.to_string(),
                    from_serde_error_to_range(&error, content),
                )
                .with_file_path("rome.json")
                .with_file_source_code(content),
            )
        } else {
            panic!("The JSON should be incorrect")
        }
    }
}
