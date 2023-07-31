use crate::WorkspaceError;
use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_deserialize::DeserializationDiagnostic;
use rome_diagnostics::{
    Advices, Category, Diagnostic, DiagnosticTags, Location, LogCategory, MessageAndDescription,
    Severity, Visit,
};
use rome_rowan::SyntaxError;
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
    Deserialization(DeserializationDiagnostic),

    /// When something is wrong with the configuration
    InvalidConfiguration(InvalidConfiguration),

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern(InvalidIgnorePattern),

    /// Thrown when there's something wrong with the files specified inside `"extends"`
    CantLoadExtendFile(CantLoadExtendFile),
}

impl From<SyntaxError> for ConfigurationDiagnostic {
    fn from(_: SyntaxError) -> Self {
        ConfigurationDiagnostic::Deserialization(DeserializationDiagnostic::new(
            markup! {"Syntax Error"},
        ))
    }
}

impl From<DeserializationDiagnostic> for ConfigurationDiagnostic {
    fn from(value: DeserializationDiagnostic) -> Self {
        ConfigurationDiagnostic::Deserialization(value)
    }
}

impl ConfigurationDiagnostic {
    pub(crate) fn new_serialization_error() -> Self {
        Self::SerializationError(SerializationError)
    }

    pub(crate) fn new_invalid_ignore_pattern(
        pattern: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidIgnorePattern(InvalidIgnorePattern {
            message: format!(
                "Couldn't parse the {}, reason: {}",
                pattern.into(),
                reason.into()
            ),
        })
    }

    pub fn new_already_exists() -> Self {
        Self::ConfigAlreadyExists(ConfigAlreadyExists {})
    }

    pub fn invalid_configuration(message: impl Display) -> Self {
        Self::InvalidConfiguration(InvalidConfiguration {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
        })
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
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.severity(),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.severity(),
        }
    }

    fn category(&self) -> Option<&'static Category> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.category(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.category(),
            ConfigurationDiagnostic::Deserialization(error) => error.category(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.category(),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.category(),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.category(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.tags(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.tags(),
            ConfigurationDiagnostic::Deserialization(error) => error.tags(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.tags(),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.tags(),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.tags(),
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.location(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.location(),
            ConfigurationDiagnostic::Deserialization(error) => error.location(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.location(),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.location(),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.location(),
        }
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.source(),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.source(),
            ConfigurationDiagnostic::Deserialization(error) => error.source(),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.source(),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.source(),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.source(),
        }
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.message(fmt),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.message(fmt),
            ConfigurationDiagnostic::Deserialization(error) => error.message(fmt),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.message(fmt),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.message(fmt),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.message(fmt),
        }
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.description(fmt),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.description(fmt),
            ConfigurationDiagnostic::Deserialization(error) => error.description(fmt),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.description(fmt),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.description(fmt),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.description(fmt),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.advices(visitor),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.advices(visitor),
            ConfigurationDiagnostic::Deserialization(error) => error.advices(visitor),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.advices(visitor),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.advices(visitor),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.advices(visitor),
        }
    }

    fn verbose_advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            ConfigurationDiagnostic::SerializationError(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::ConfigAlreadyExists(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::Deserialization(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::InvalidIgnorePattern(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::CantLoadExtendFile(error) => error.verbose_advices(visitor),
            ConfigurationDiagnostic::InvalidConfiguration(error) => error.verbose_advices(visitor),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigurationAdvices {
    messages: Vec<MarkupBuf>,
}

impl Advices for ConfigurationAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for message in &self.messages {
            visitor.record_log(LogCategory::Info, message)?;
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
    severity = Error,
)]
pub struct InvalidIgnorePattern {
    #[message]
    #[description]
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
	category = "configuration",
	severity = Error,
)]
pub struct CantLoadExtendFile {
    #[location(resource)]
    file_path: String,
    #[message]
    #[description]
    message: MessageAndDescription,

    #[verbose_advice]
    verbose_advice: ConfigurationAdvices,
}

impl CantLoadExtendFile {
    pub fn new(file_path: impl Into<String>, message: impl Display) -> Self {
        Self {
            file_path: file_path.into(),
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
            verbose_advice: ConfigurationAdvices::default(),
        }
    }

    pub fn with_verbose_advice(mut self, messsage: impl Display) -> Self {
        self.verbose_advice
            .messages
            .push(markup! {{messsage}}.to_owned());
        self
    }
}

impl From<CantLoadExtendFile> for WorkspaceError {
    fn from(value: CantLoadExtendFile) -> Self {
        WorkspaceError::Configuration(ConfigurationDiagnostic::CantLoadExtendFile(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
	category = "configuration",
	severity = Error,
)]
pub struct InvalidConfiguration {
    #[message]
    #[description]
    message: MessageAndDescription,
}

#[cfg(test)]
mod test {
    use crate::configuration::diagnostics::ConfigurationDiagnostic;
    use crate::{Configuration, MatchOptions, Matcher};
    use rome_deserialize::json::deserialize_from_json_str;
    use rome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error};
    use rome_json_parser::JsonParserOptions;

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
        assert_eq!(std::mem::size_of::<ConfigurationDiagnostic>(), 104);
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
        let result =
            deserialize_from_json_str::<Configuration>(content, JsonParserOptions::default());

        assert!(result.has_errors());
        for diagnostic in result.into_diagnostics() {
            snap_diagnostic("deserialization_error", diagnostic)
        }
    }

    #[test]
    fn deserialization_quick_check() {
        let content = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noDebugger": {
                "level": "off",
                "options": { "hooks": [] }
            }
        }
    }
  }
}"#;
        let _result =
            deserialize_from_json_str::<Configuration>(content, JsonParserOptions::default())
                .into_deserialized();
    }
}
