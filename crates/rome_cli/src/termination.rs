use rome_console::fmt::{Display, Formatter};
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::adapters::{IoError, PicoArgsError};
use rome_diagnostics::{
    category, Advices, Category, Diagnostic, DiagnosticTags, Error, Location, LogCategory,
    MessageAndDescription, Resource, Severity, Visit,
};
use rome_service::RomeError;
use rome_text_size::TextRange;
use std::{env::current_exe, fmt::Debug};

fn command_name() -> String {
    current_exe()
        .ok()
        .and_then(|path| Some(path.file_name()?.to_str()?.to_string()))
        .unwrap_or_else(|| String::from("rome"))
}

#[derive(Debug)]
pub struct TerminationDiagnostic {
    span: Option<TextRange>,
    path: Resource<&'static str>,
    message: MessageAndDescription,
    advices: TerminationAdvice,
    source: Option<Error>,
    category: &'static Category,
    tags: DiagnosticTags,
    severity: Severity,
    is_workspace_error: bool,
}

#[derive(Debug, Default)]
struct TerminationAdvice {
    notes: Vec<MarkupBuf>,
}

impl Advices for TerminationAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for note in &self.notes {
            visitor.record_log(LogCategory::Info, note)?;
        }
        Ok(())
    }
}

impl TerminationDiagnostic {
    pub fn new(category: &'static Category, message: impl Display) -> Self {
        Self {
            category,
            span: None,
            path: Resource::Memory,
            message: MessageAndDescription::from(
                markup! {
                    {message}
                }
                .to_owned(),
            ),
            advices: TerminationAdvice::default(),
            source: None,
            tags: DiagnosticTags::FIXABLE,
            severity: Severity::Error,
            is_workspace_error: false,
        }
    }

    /// Returned when a subcommand is called with an unsupported combination of arguments
    pub fn new_incompatible_arguments(
        first_argument: impl Display,
        second_argument: impl Display,
    ) -> Self {
        Self::new(
            category!("internalError/io"),
            markup! {
                "Incompatible arguments "<Emphasis>{first_argument}</Emphasis>" and "<Emphasis>{second_argument}</Emphasis>
            },
        ).with_resource(Resource::Argv)
    }

    pub fn new_parse(argument: impl Into<String>, source: pico_args::Error) -> Self {
        let argument = argument.into();
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Cannot parse the argument "<Emphasis>{argument}</Emphasis>
            },
        )
        .with_resource(Resource::Argv)
        .with_source(Error::from(PicoArgsError::from(source)))
    }

    /// Returned when it is called with a subcommand it doesn't know
    pub fn new_unknown_command(command: impl Display) -> Self {
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Unknown command "<Emphasis>{command}</Emphasis>
            },
        )
        .with_resource(Resource::Argv)
    }

    /// Returned when a subcommand is called without any arguments
    pub fn new_empty_arguments() -> Self {
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Empty arguments"
            },
        )
        .with_resource(Resource::Argv)
    }

    /// Returned when a required argument is not present in the command line
    pub fn new_missing_argument(argument: impl Display, subcommand: impl Display) -> Self {
        let command_name = command_name();
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Missing argument "<Emphasis>{argument}</Emphasis>
            },
        )
        .with_resource(Resource::Argv)
        .with_note(markup! {
            "Type "<Italic>{command_name}" "{subcommand}" --help"</Italic>" for more information."
        })
    }

    pub fn new_no_files_processed() -> Self {
        Self::new(
            category!("flags/invalid"),
            markup! {
                "No files were processed in the specified paths."
            },
        )
    }

    /// Returned when the CLI  doesn't recognize a command line argument
    pub fn new_unexpected_argument(argument: impl Display, subcommand: impl Display) -> Self {
        let command_name = command_name();
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Unrecognized option "<Emphasis>{argument}</Emphasis>"."
            },
        )
        .with_resource(Resource::Argv)
        .with_note(markup! {
            "Type "<Italic>{command_name}" "{subcommand}" --help"</Italic>" for more information."
        })
    }

    pub fn new_workspace(error: RomeError) -> Self {
        let mut item = Self::new(
            category!("configuration"),
            markup! {
                "Workspace error."
            },
        )
        .with_source(Error::from(error));

        item.is_workspace_error = true;
        item
    }

    pub fn new_io(error: std::io::Error) -> Self {
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Errors occurred while executing I/O operations."
            },
        )
        .with_resource(Resource::Memory)
        .with_source(Error::from(IoError::from(error)))
    }

    pub fn new_check() -> Self {
        Self::new(
            category!("internalError/io"),
            markup! {
                "Some errors were emitted while running checks"
            },
        )
    }

    pub fn new_server_not_running() -> Self {
        let command_name = command_name();
        Self::new(
            category!("internalError/io"),
            markup! {
                "No running instance of the Rome daemon server was found."
            },
        )
        .with_resource(Resource::Memory)
        .with_note(markup! {
            "Run "<Italic>{command_name}" start"</Italic>" to start a server."
        })
    }

    pub fn new_incompatible_configuration(reason: impl Display) -> Self {
        Self::new(
            category!("flags/invalid"),
            markup! {
                "The combination of configuration and arguments is invalid: \n "{{reason}}
            },
        )
    }

    pub fn new_overflown(argument: impl Into<String>, maximum: u16) -> Self {
        let argument = argument.into();
        Self::new(
            category!("flags/invalid"),
            markup! {
                 "The value of the argument "<Emphasis>{argument}</Emphasis>" is too high, maximum accepted "{{maximum}}
            }
        )
            .with_resource(Resource::Argv)
    }

    /// Return by the help command when it is called with a subcommand it doesn't know
    pub fn new_unknown_help(command: impl Display) -> Self {
        Self::new(
            category!("flags/invalid"),
            markup! {
                "Cannot print help for unknown command "<Emphasis>{command}</Emphasis>
            },
        )
        .with_resource(Resource::Argv)
    }

    fn with_resource(mut self, resource: Resource<&'static str>) -> Self {
        self.path = resource;
        self
    }

    fn with_note(mut self, message: impl Display) -> Self {
        self.advices.notes.push(markup! {{message}}.to_owned());
        self
    }

    fn with_source(mut self, error: Error) -> Self {
        self.source = Some(error);
        self
    }
}

impl Diagnostic for TerminationDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        if self.is_workspace_error {
            if let Some(source) = self.source.as_ref() {
                return source.category();
            }
        }
        Some(self.category)
    }

    fn tags(&self) -> DiagnosticTags {
        if self.is_workspace_error {
            if let Some(source) = self.source.as_ref() {
                return source.tags();
            }
        }
        self.tags
    }

    fn severity(&self) -> Severity {
        if self.is_workspace_error {
            if let Some(source) = self.source.as_ref() {
                return source.severity();
            }
        }
        self.severity
    }

    fn location(&self) -> Location<'_> {
        if self.is_workspace_error {
            if let Some(source) = self.source.as_ref() {
                return source.location();
            }
        }
        Location::builder().span(&self.span).build()
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        if self.is_workspace_error {
            if let Some(source) = self.source.as_ref() {
                return source.message(fmt);
            }
        }
        rome_console::fmt::Display::fmt(&self.message, fmt)
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if self.is_workspace_error {
            if let Some(source) = self.source.as_ref() {
                return source.advices(visitor);
            }
        }
        self.advices.record(visitor)
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        if self.is_workspace_error {
            return self.source.as_ref().and_then(|error| error.source());
        }
        self.source.as_ref().map(|source| source.as_ref())
    }
}

impl From<RomeError> for TerminationDiagnostic {
    fn from(error: RomeError) -> Self {
        TerminationDiagnostic::new_workspace(error)
    }
}

impl From<std::io::Error> for TerminationDiagnostic {
    fn from(error: std::io::Error) -> Self {
        TerminationDiagnostic::new_io(error)
    }
}

#[cfg(test)]
mod test {
    use crate::TerminationDiagnostic;

    #[test]
    fn termination_diagnostic_size() {
        assert_eq!(
            std::mem::size_of::<TerminationDiagnostic>(),
            136,
            "you successfully decreased the size of the diagnostic!"
        )
    }
}
