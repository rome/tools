use crate::file_handlers::Language;
use crate::ConfigurationDiagnostic;
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
pub enum WorkspaceError {
    /// Can't export the report of the CLI into a file
    ReportNotSerializable(ReportNotSerializable),
    /// The project contains uncommitted changes
    DirtyWorkspace(DirtyWorkspace),
    /// The file does not exist in the [crate::Workspace]
    NotFound(NotFound),
    /// A file is not supported. It contains the language and path of the file
    /// Use this error if Rome is trying to process a file that Rome can't understand
    SourceFileNotSupported(SourceFileNotSupported),
    /// The formatter encountered an error while formatting the file
    FormatError(FormatError),
    /// The file could not be formatted since it has syntax errors and `format_with_errors` is disabled
    FormatWithErrorsDisabled(FormatWithErrorsDisabled),
    /// The file could not be analyzed because a rule caused an error.
    RuleError(RuleError),
    /// Thrown when Rome can't read a generic directory
    CantReadDirectory(CantReadDirectory),
    /// Thrown when Rome can't read a generic file
    CantReadFile(CantReadFile),
    /// Error thrown when validating the configuration. Once deserialized, further checks have to be done.
    Configuration(ConfigurationDiagnostic),
    /// Error thrown when Rome cannot rename a symbol.
    RenameError(RenameError),
    /// Error emitted by the underlying transport layer for a remote Workspace
    TransportError(TransportError),
    /// Emitted when the file is ignored and should not be processed
    FileIgnored(FileIgnored),
    /// Emitted when a file could not be parsed because it's larger than the size limit
    FileTooLarge(FileTooLarge),
}

impl WorkspaceError {
    pub fn format_with_errors_disabled() -> Self {
        Self::FormatWithErrorsDisabled(FormatWithErrorsDisabled)
    }

    pub fn cant_read_file(path: String) -> Self {
        Self::CantReadFile(CantReadFile { path })
    }

    pub fn not_found() -> Self {
        Self::NotFound(NotFound)
    }

    pub fn file_too_large(path: String, size: usize, limit: usize) -> Self {
        Self::FileTooLarge(FileTooLarge { path, size, limit })
    }

    pub fn file_ignored(path: String) -> Self {
        Self::FileIgnored(FileIgnored { path })
    }

    pub fn source_file_not_supported(
        language: Language,
        path: String,
        extension: Option<String>,
    ) -> Self {
        Self::SourceFileNotSupported(SourceFileNotSupported {
            language,
            path,
            extension,
        })
    }

    pub fn report_not_serializable(reason: impl Into<String>) -> Self {
        Self::ReportNotSerializable(ReportNotSerializable {
            reason: reason.into(),
        })
    }
}

impl Error for WorkspaceError {}

impl Debug for WorkspaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for WorkspaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Diagnostic::description(self, f)
    }
}

impl Termination for WorkspaceError {
    fn report(self) -> ExitCode {
        ExitCode::FAILURE
    }
}

impl Diagnostic for WorkspaceError {
    fn category(&self) -> Option<&'static Category> {
        match self {
            WorkspaceError::FormatWithErrorsDisabled(error) => error.category(),
            WorkspaceError::FormatError(err) => err.category(),
            WorkspaceError::RuleError(error) => error.category(),
            WorkspaceError::Configuration(error) => error.category(),
            WorkspaceError::RenameError(error) => error.category(),
            WorkspaceError::TransportError(error) => error.category(),
            WorkspaceError::ReportNotSerializable(error) => error.category(),
            WorkspaceError::NotFound(error) => error.category(),
            WorkspaceError::DirtyWorkspace(error) => error.category(),
            WorkspaceError::SourceFileNotSupported(error) => error.category(),
            WorkspaceError::CantReadDirectory(error) => error.category(),
            WorkspaceError::CantReadFile(error) => error.category(),
            WorkspaceError::FileIgnored(error) => error.category(),
            WorkspaceError::FileTooLarge(error) => error.category(),
        }
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceError::FormatWithErrorsDisabled(error) => error.description(fmt),
            WorkspaceError::FormatError(error) => Diagnostic::description(error, fmt),
            WorkspaceError::RuleError(error) => Diagnostic::description(error, fmt),
            WorkspaceError::Configuration(error) => error.description(fmt),
            WorkspaceError::RenameError(error) => error.description(fmt),
            WorkspaceError::TransportError(error) => error.description(fmt),
            WorkspaceError::ReportNotSerializable(error) => error.description(fmt),
            WorkspaceError::NotFound(error) => error.description(fmt),
            WorkspaceError::DirtyWorkspace(error) => error.description(fmt),
            WorkspaceError::SourceFileNotSupported(error) => error.description(fmt),
            WorkspaceError::CantReadDirectory(error) => error.description(fmt),
            WorkspaceError::CantReadFile(error) => error.description(fmt),
            WorkspaceError::FileIgnored(error) => error.description(fmt),
            WorkspaceError::FileTooLarge(error) => error.description(fmt),
        }
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            WorkspaceError::FormatWithErrorsDisabled(error) => error.message(fmt),
            WorkspaceError::FormatError(err) => err.message(fmt),
            WorkspaceError::RuleError(error) => error.message(fmt),
            WorkspaceError::Configuration(error) => error.message(fmt),
            WorkspaceError::RenameError(error) => error.message(fmt),
            WorkspaceError::TransportError(error) => error.message(fmt),
            WorkspaceError::ReportNotSerializable(error) => error.message(fmt),
            WorkspaceError::NotFound(error) => error.message(fmt),
            WorkspaceError::DirtyWorkspace(error) => error.message(fmt),
            WorkspaceError::SourceFileNotSupported(error) => error.message(fmt),
            WorkspaceError::CantReadDirectory(error) => error.message(fmt),
            WorkspaceError::CantReadFile(error) => error.message(fmt),
            WorkspaceError::FileIgnored(error) => error.message(fmt),
            WorkspaceError::FileTooLarge(error) => error.message(fmt),
        }
    }

    fn severity(&self) -> Severity {
        match self {
            WorkspaceError::FormatError(err) => err.severity(),
            WorkspaceError::RuleError(error) => error.severity(),
            WorkspaceError::Configuration(error) => error.severity(),
            WorkspaceError::RenameError(error) => error.severity(),
            WorkspaceError::TransportError(error) => error.severity(),
            WorkspaceError::ReportNotSerializable(error) => error.severity(),
            WorkspaceError::DirtyWorkspace(error) => error.severity(),
            WorkspaceError::NotFound(error) => error.severity(),
            WorkspaceError::SourceFileNotSupported(error) => error.severity(),
            WorkspaceError::FormatWithErrorsDisabled(error) => error.severity(),
            WorkspaceError::CantReadDirectory(error) => error.severity(),
            WorkspaceError::CantReadFile(error) => error.severity(),
            WorkspaceError::FileIgnored(error) => error.severity(),
            WorkspaceError::FileTooLarge(error) => error.severity(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            WorkspaceError::FormatError(err) => err.tags(),
            WorkspaceError::RuleError(error) => error.tags(),
            WorkspaceError::Configuration(error) => error.tags(),
            WorkspaceError::RenameError(error) => error.tags(),
            WorkspaceError::TransportError(error) => error.tags(),
            WorkspaceError::ReportNotSerializable(error) => error.tags(),
            WorkspaceError::DirtyWorkspace(error) => error.tags(),
            WorkspaceError::NotFound(error) => error.tags(),
            WorkspaceError::SourceFileNotSupported(error) => error.tags(),
            WorkspaceError::FormatWithErrorsDisabled(error) => error.tags(),
            WorkspaceError::CantReadDirectory(error) => error.tags(),
            WorkspaceError::CantReadFile(error) => error.tags(),
            WorkspaceError::FileIgnored(error) => error.tags(),
            WorkspaceError::FileTooLarge(error) => error.tags(),
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            WorkspaceError::FormatError(err) => err.location(),
            WorkspaceError::RuleError(error) => error.location(),
            WorkspaceError::Configuration(error) => error.location(),
            WorkspaceError::RenameError(error) => error.location(),
            WorkspaceError::TransportError(error) => error.location(),
            WorkspaceError::ReportNotSerializable(error) => error.location(),
            WorkspaceError::DirtyWorkspace(error) => error.location(),
            WorkspaceError::NotFound(error) => error.location(),
            WorkspaceError::SourceFileNotSupported(error) => error.location(),
            WorkspaceError::FormatWithErrorsDisabled(error) => error.location(),
            WorkspaceError::CantReadDirectory(error) => error.location(),
            WorkspaceError::CantReadFile(error) => error.location(),
            WorkspaceError::FileIgnored(error) => error.location(),
            WorkspaceError::FileTooLarge(error) => error.location(),
        }
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        match self {
            WorkspaceError::FormatError(error) => Diagnostic::source(error),
            WorkspaceError::RuleError(error) => Diagnostic::source(error),
            WorkspaceError::Configuration(error) => Diagnostic::source(error),
            WorkspaceError::RenameError(error) => Diagnostic::source(error),
            WorkspaceError::TransportError(error) => Diagnostic::source(error),
            WorkspaceError::ReportNotSerializable(error) => Diagnostic::source(error),
            WorkspaceError::DirtyWorkspace(error) => Diagnostic::source(error),
            WorkspaceError::NotFound(error) => Diagnostic::source(error),
            WorkspaceError::SourceFileNotSupported(error) => Diagnostic::source(error),
            WorkspaceError::FormatWithErrorsDisabled(error) => Diagnostic::source(error),
            WorkspaceError::CantReadDirectory(error) => Diagnostic::source(error),
            WorkspaceError::CantReadFile(error) => Diagnostic::source(error),
            WorkspaceError::FileIgnored(error) => Diagnostic::source(error),
            WorkspaceError::FileTooLarge(error) => Diagnostic::source(error),
        }
    }
}

impl From<FormatError> for WorkspaceError {
    fn from(err: FormatError) -> Self {
        Self::FormatError(err)
    }
}

impl From<TransportError> for WorkspaceError {
    fn from(err: TransportError) -> Self {
        Self::TransportError(err)
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message = "Uncommitted changes in repository"
)]
pub struct DirtyWorkspace;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("The report can't be serialized, here's why: "{self.reason}),
        description = "The report can't be serialized, here's why: {reason}"
    )
)]
pub struct ReportNotSerializable {
    reason: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message = "The file does not exist in the workspace.",
    tags(INTERNAL)
)]
pub struct NotFound;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "format",
    message = "The file does not exist in the workspace."
)]
pub struct FormatWithErrorsDisabled;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("Rome couldn't read the following directory, maybe for permissions reasons or it doesn't exists: "{self.path}),
        description = "Rome couldn't read the following directory, maybe for permissions reasons or it doesn't exists: {path}"
    )
)]
pub struct CantReadDirectory {
    #[location(resource)]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("Rome couldn't read the following file, maybe for permissions reasons or it doesn't exists: "{self.path}),
        description = "Rome couldn't read the following file, maybe for permissions reasons or it doesn't exists: {path}"
    )
)]
pub struct CantReadFile {
    #[location(resource)]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("The file "{self.path}" was ignored"),
        description = "The file {path} was ignored"
    )
)]
pub struct FileIgnored {
    #[location(resource)]
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTooLarge {
    path: String,
    size: usize,
    limit: usize,
}

impl Diagnostic for FileTooLarge {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/fs"))
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(
            markup!{
                "Size of "{self.path}" is "{Bytes(self.size)}" which exceeds configured maximum of "{Bytes(self.limit)}" for this project. The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't."
            }
        )
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "Size of {} is {} which exceeds configured maximum of {} for this project. \
               The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't.",
               self.path, Bytes(self.size), Bytes(self.limit)
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFileNotSupported {
    language: Language,
    path: String,
    extension: Option<String>,
}

impl Diagnostic for SourceFileNotSupported {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        if self.language != Language::Unknown {
            fmt.write_markup(markup! {
                "Rome doesn't support this feature for the language "{{&self.language}}
            })
        } else if let Some(ext) = self.extension.as_ref() {
            fmt.write_markup(markup! {
                "Rome could not determine the language for the file extension "{{ext}}
            })
        } else {
            fmt.write_markup(
                markup!{
                    "Rome could not determine the language for the file "{self.path}" because it doesn't have a clear extension"
                }
            )
        }
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

impl Display for TransportError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.description(fmt)
    }
}

impl Diagnostic for TransportError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
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
    use crate::diagnostics::{
        CantReadDirectory, CantReadFile, DirtyWorkspace, FileIgnored, FileTooLarge, NotFound,
        SourceFileNotSupported,
    };
    use crate::file_handlers::Language;
    use crate::{TransportError, WorkspaceError};
    use rome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error};
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
    fn diagnostic_size() {
        assert_eq!(std::mem::size_of::<WorkspaceError>(), 88)
    }

    #[test]
    fn dirty_workspace() {
        snap_diagnostic(
            "dirty_workspace",
            WorkspaceError::DirtyWorkspace(DirtyWorkspace).into(),
        )
    }

    #[test]
    fn file_ignored() {
        snap_diagnostic(
            "file_ignored",
            WorkspaceError::FileIgnored(FileIgnored {
                path: "example.js".to_string(),
            })
            .with_file_path("example.js"),
        )
    }

    #[test]
    fn cant_read_directory() {
        snap_diagnostic(
            "cant_read_directory",
            WorkspaceError::CantReadDirectory(CantReadDirectory {
                path: "example/".to_string(),
            })
            .with_file_path("example/"),
        )
    }

    #[test]
    fn cant_read_file() {
        snap_diagnostic(
            "cant_read_file",
            WorkspaceError::CantReadFile(CantReadFile {
                path: "example.js".to_string(),
            })
            .with_file_path("example.js"),
        )
    }

    #[test]
    fn not_found() {
        snap_diagnostic(
            "not_found",
            WorkspaceError::NotFound(NotFound).with_file_path("not_found.js"),
        )
    }

    #[test]
    fn source_file_not_supported() {
        let path = RomePath::new("not_supported.toml");
        snap_diagnostic(
            "source_file_not_supported",
            WorkspaceError::SourceFileNotSupported(SourceFileNotSupported {
                language: Language::Unknown,
                path: path.display().to_string(),
                extension: path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.to_string()),
            })
            .with_file_path("not_supported.toml"),
        )
    }

    #[test]
    fn file_too_large() {
        snap_diagnostic(
            "file_too_large",
            WorkspaceError::FileTooLarge(FileTooLarge {
                path: "example.js".to_string(),
                limit: 100,
                size: 500,
            })
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
            WorkspaceError::FormatError(FormatError::SyntaxError).with_file_path("example.js"),
        )
    }
}
