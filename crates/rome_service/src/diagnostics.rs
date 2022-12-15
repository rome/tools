use crate::file_handlers::Language;
use crate::ConfigurationError;
use rome_console::fmt::Bytes;
use rome_console::markup;
use rome_diagnostics::{category, Category, Diagnostic, DiagnosticTags, Location, Severity};
use rome_formatter::FormatError;
use rome_js_analyze::utils::rename::RenameError;
use rome_js_analyze::RuleError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::process::{ExitCode, Termination};

#[derive(Serialize, Deserialize)]
/// Generic errors thrown during rome operations
pub enum RomeError {
    /// Can't export the report of the CLI into a file
    ReportNotSerializable(String),
    /// The project contains uncommitted changes
    DirtyWorkspace,
    /// The file does not exist in the [crate::Workspace]
    NotFound,
    /// A file is not supported. It contains the language and path of the file
    /// Use this error if Rome is trying to process a file that Rome can't understand
    SourceFileNotSupported {
        language: Language,
        path: String,
        extension: Option<String>,
    },
    /// The formatter encountered an error while formatting the file
    FormatError(FormatError),
    /// The file could not be formatted since it has syntax errors and `format_with_errors` is disabled
    FormatWithErrorsDisabled,
    /// The file could not be analyzed because a rule caused an error.
    RuleError(RuleError),
    /// Thrown when Rome can't read a generic directory
    CantReadDirectory(String),
    /// Thrown when Rome can't read a generic file
    CantReadFile(String),
    /// Error thrown when validating the configuration. Once deserialized, further checks have to be done.
    Configuration(ConfigurationError),
    /// Error thrown when Rome cannot rename a symbol.
    RenameError(RenameError),
    /// Error emitted by the underlying transport layer for a remote Workspace
    TransportError(TransportError),
    /// Emitted when the file is ignored and should not be processed
    FileIgnored(String),
    /// Emitted when a file could not be parsed because it's larger than the size limite
    FileTooLarge {
        path: String,
        size: usize,
        limit: usize,
    },
}

impl Debug for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Diagnostic::description(self, f)
    }
}

impl Error for RomeError {}

impl Termination for RomeError {
    fn report(self) -> ExitCode {
        ExitCode::FAILURE
    }
}

impl Diagnostic for RomeError {
    fn category(&self) -> Option<&'static Category> {
        match self {
            RomeError::FormatWithErrorsDisabled => Some(category!("format")),
            RomeError::FormatError(err) => err.category(),
            RomeError::RuleError(error) => error.category(),
            RomeError::Configuration(error) => error.category(),
            RomeError::RenameError(error) => error.category(),
            RomeError::TransportError(error) => error.category(),
            RomeError::ReportNotSerializable(_) => Some(category!("internalError/io")),
            RomeError::NotFound
            | RomeError::DirtyWorkspace
            | RomeError::SourceFileNotSupported { .. }
            | RomeError::CantReadDirectory(_)
            | RomeError::CantReadFile(_)
            | RomeError::FileIgnored(_)
            | RomeError::FileTooLarge { .. } => Some(category!("internalError/fs")),
        }
    }

    fn description(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RomeError::SourceFileNotSupported {
                language,
                path,
                extension,
            } => {
                if *language != Language::Unknown {
                    write!(
                        f,
                        "Rome doesn't support this feature for the language {language:?}"
                    )
                } else if let Some(ext) = extension {
                    write!(
                        f,
                        "Rome could not determine the language for the file extension {ext}"
                    )
                } else {
                    write!(
                f,
                "Rome could not determine the language for the file {path} because it doesn't have a clear extension"
                )
                }
            }
            RomeError::NotFound => {
                write!(f, "the file does not exist in the workspace")
            }
            RomeError::FormatError(cause) => {
                write!(
                    f,
                    "the formatter encountered an error while formatting the file: {}",
                    cause
                )
            }
            RomeError::FormatWithErrorsDisabled => {
                write!(f, "the file could not be formatted since it has syntax errors and `format_with_errors` is disabled")
            }
            RomeError::CantReadDirectory(path) => {
                write!(
                f,
                "Rome couldn't read the following directory, maybe for permissions reasons or it doesn't exists: {}",
                path
                )
            }
            RomeError::CantReadFile(path) => {
                write!(
                f,
                "Rome couldn't read the following file, maybe for permissions reasons or it doesn't exists: {}",
                path
                )
            }

            RomeError::Configuration(error) => fmt::Display::fmt(error, f),
            RomeError::DirtyWorkspace => {
                write!(f, "Uncommitted changes in repository")
            }
            RomeError::RenameError(error) => fmt::Display::fmt(error, f),
            RomeError::RuleError(cause) => {
                write!(
                    f,
                    "the linter encountered an error while analyzing the file: {cause}",
                )
            }

            RomeError::TransportError(err) => {
                write!(f, "{err}",)
            }
            RomeError::FileIgnored(path) => {
                write!(f, "The file {} was ignored", path)
            }
            RomeError::FileTooLarge { path, size, limit } => {
                write!(f, "Size of {} is {} which exceeds configured maximum of {} for this project. The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't.", path, Bytes(*size), Bytes(*limit))
            }
            RomeError::ReportNotSerializable(reason) => {
                write!(f, "The report can't be serialized, here's why: \n{reason}")
            }
        }
    }

    fn message(&self, f: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            RomeError::DirtyWorkspace => {
                f.write_markup(markup! { "Uncommitted changes in repository" })
            }
            RomeError::NotFound => {
                f.write_markup(markup! { "The file does not exist in the workspace." })
            }
            RomeError::SourceFileNotSupported{ language, path, extension} => {
                if *language != Language::Unknown {
                    f.write_markup(
                        markup! { "Rome doesn't support this feature for the language "{{language}}"" }
                    )
                } else if let Some(ext) = extension {
                    f.write_markup(markup! {
                        "Rome could not determine the language for the file extension "{{ext}}""
                    })
                } else {
                    f.write_markup(
                        markup!{
                            "Rome could not determine the language for the file "{{path}}" because it doesn't have a clear extension"
                        }
                    )
                }
            }
            RomeError::FormatError(err) => {
                f.write_markup(markup! {
                    "the formatter encountered an error while formatting the file: "
                })?;
                err.message(f)
            },
            RomeError::FormatWithErrorsDisabled => f.write_markup(
                markup!{  "The file could not be formatted since it has syntax errors and "<Emphasis>"formatWithErrors"</Emphasis>" is disabled" }
            ),
            RomeError::RuleError(error) => error.message(f),
            RomeError::CantReadDirectory(path) => {
                f.write_markup(
                    markup!{
                        "Rome couldn't read the following directory, maybe for permissions reasons or it doesn't exists: "{{path}}

                    }
                )
            }
            RomeError::CantReadFile(path) => {
                f.write_markup(
                    markup!{
                        "Rome couldn't read the following file, maybe for permissions reasons or it doesn't exists: "{{path}}

                    }
                )
            },
            RomeError::Configuration(error) => error.message(f),
            RomeError::RenameError(error) => error.message(f),
            RomeError::TransportError(error) => error.message(f),
            RomeError::FileIgnored(path) => {
                write!(f, "The file {} was ignored", path)
            },
            RomeError::FileTooLarge { path, size, limit } => {
                f.write_markup(
                    markup!{
            "Size of "{{path}}" is "{{Bytes(*size)}}" which exceeds configured maximum of "{{Bytes(*limit)}}" for this project. The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't."
                    }
                )
            },

            RomeError::ReportNotSerializable(reason) => {
                f.write_markup(
                    markup!{
                         "The report can't be serialized, here's why: \n"{{reason}}
                    }
                )
            }
        }
    }

    fn severity(&self) -> Severity {
        match self {
            RomeError::FormatError(err) => err.severity(),
            RomeError::RuleError(error) => error.severity(),
            RomeError::Configuration(error) => error.severity(),
            RomeError::RenameError(error) => error.severity(),
            RomeError::TransportError(error) => error.severity(),
            RomeError::DirtyWorkspace
            | RomeError::NotFound
            | RomeError::ReportNotSerializable(_)
            | RomeError::SourceFileNotSupported { .. }
            | RomeError::FormatWithErrorsDisabled
            | RomeError::CantReadFile(_)
            | RomeError::CantReadDirectory(_)
            | RomeError::FileIgnored(_)
            | RomeError::FileTooLarge { .. } => Severity::Error,
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            RomeError::FormatError(err) => err.tags(),
            RomeError::RuleError(error) => error.tags(),
            RomeError::Configuration(error) => error.tags(),
            RomeError::RenameError(error) => error.tags(),
            RomeError::TransportError(error) => error.tags(),
            RomeError::NotFound => DiagnosticTags::INTERNAL,
            RomeError::ReportNotSerializable(_) => DiagnosticTags::INTERNAL,
            _ => DiagnosticTags::FIXABLE,
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            RomeError::FormatError(err) => err.location(),
            RomeError::RuleError(error) => error.location(),
            RomeError::Configuration(error) => error.location(),
            RomeError::RenameError(error) => error.location(),
            RomeError::TransportError(error) => error.location(),
            RomeError::SourceFileNotSupported { path, .. } => {
                Location::builder().resource(path).build()
            }
            RomeError::CantReadFile(path)
            | RomeError::FileIgnored(path)
            | RomeError::CantReadDirectory(path)
            | RomeError::FileTooLarge { path, .. } => Location::builder().resource(path).build(),
            _ => Location::builder().build(),
        }
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        match self {
            RomeError::FormatError(error) => Diagnostic::source(error),
            RomeError::RuleError(error) => Diagnostic::source(error),
            RomeError::Configuration(error) => Diagnostic::source(error),
            RomeError::RenameError(error) => Diagnostic::source(error),
            RomeError::TransportError(error) => Diagnostic::source(error),
            _ => None,
        }
    }
}

impl From<FormatError> for RomeError {
    fn from(err: FormatError) -> Self {
        Self::FormatError(err)
    }
}

impl From<TransportError> for RomeError {
    fn from(err: TransportError) -> Self {
        Self::TransportError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Error emitted by the underlying transport layer for a remote Workspace
pub enum TransportError {
    /// Error emitted by the transport layer if the connection was lost due to an I/O error
    ChannelClosed,
    /// Error emitted by the transport layer if a request timed out
    Timeout,
    /// Error caused by a serialization or deserialization issue
    SerdeError(String),
    /// Generic error type for RPC errors that can't be deserialized into RomeError
    RPCError(String),
}

impl Error for TransportError {}

impl Display for TransportError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            TransportError::SerdeError(err) => write!(fmt, "serialization error: {err}"),
            TransportError::ChannelClosed => fmt.write_str(
                "a request to the remote workspace failed because the connection was interrupted",
            ),
            TransportError::Timeout => {
                fmt.write_str("the request to the remote workspace timed out")
            }
            TransportError::RPCError(err) => fmt.write_str(err),
        }
    }
}

impl Diagnostic for TransportError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            TransportError::SerdeError(err) => write!(fmt, "serialization error: {err}"),
            TransportError::ChannelClosed => fmt.write_str(
                "a request to the remote workspace failed because the connection was interrupted",
            ),
            TransportError::Timeout => {
                fmt.write_str("the request to the remote workspace timed out")
            }
            TransportError::RPCError(err) => fmt.write_str(err),
        }
    }
    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::INTERNAL
    }
}

#[cfg(test)]
mod test {
    use crate::file_handlers::Language;
    use crate::{RomeError, TransportError};
    use rome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error, FileId};
    use rome_formatter::FormatError;
    use rome_fs::RomePath;
    use std::ffi::OsStr;

    fn snap_diagnostic(test_name: &str, diagnostic: Error) {
        let content = print_diagnostic_to_string(&diagnostic);

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);

        });
    }

    #[test]
    fn dirty_workspace() {
        snap_diagnostic("dirty_workspace", RomeError::DirtyWorkspace.into())
    }

    #[test]
    fn file_ignored() {
        snap_diagnostic(
            "file_ignored",
            RomeError::FileIgnored("example.js".to_string()).with_file_path("example.js"),
        )
    }

    #[test]
    fn cant_read_directory() {
        snap_diagnostic(
            "cant_read_directory",
            RomeError::CantReadDirectory("example/".to_string()).with_file_path("example/"),
        )
    }

    #[test]
    fn cant_read_file() {
        snap_diagnostic(
            "cant_read_file",
            RomeError::CantReadFile("example.js".to_string()).with_file_path("example.js"),
        )
    }

    #[test]
    fn not_found() {
        snap_diagnostic(
            "not_found",
            RomeError::NotFound.with_file_path("not_found.js"),
        )
    }

    #[test]
    fn source_file_not_supported() {
        let path = RomePath::new("not_supported.toml", FileId::zero());
        snap_diagnostic(
            "source_file_not_supported",
            RomeError::SourceFileNotSupported {
                language: Language::Unknown,
                path: path.display().to_string(),
                extension: path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.to_string()),
            }
            .with_file_path("not_supported.toml"),
        )
    }

    #[test]
    fn file_too_large() {
        snap_diagnostic(
            "file_too_large",
            RomeError::FileTooLarge {
                path: "example.js".to_string(),
                limit: 100,
                size: 500,
            }
            .with_file_path("example.js"),
        )
    }

    #[test]
    fn transport_channel_closed() {
        snap_diagnostic(
            "transport_channel_closed",
            TransportError::ChannelClosed.into(),
        )
    }

    #[test]
    fn transport_timeout() {
        snap_diagnostic("transport_timeout", TransportError::Timeout.into())
    }

    #[test]
    fn transport_rpc_error() {
        snap_diagnostic(
            "transport_rpc_error",
            TransportError::RPCError("Some generic error".to_string()).into(),
        )
    }

    #[test]
    fn transport_serde_error() {
        snap_diagnostic(
            "transport_serde_error",
            TransportError::SerdeError("Some serialization/deserialization error".to_string())
                .into(),
        )
    }

    #[test]
    fn formatter_syntax_error() {
        snap_diagnostic(
            "formatter_syntax_error",
            RomeError::FormatError(FormatError::SyntaxError).with_file_path("example.js"),
        )
    }
}
