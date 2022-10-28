use rome_console::fmt::Bytes;
use rome_console::{Console, EnvConsole};
use rome_formatter::FormatError;
use rome_fs::{FileSystem, OsFileSystem, RomePath};
use rome_js_analyze::utils::rename::RenameError;
use rome_js_analyze::RuleError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

pub mod configuration;
mod file_handlers;
pub mod settings;
pub mod workspace;

pub mod matcher;

#[cfg(feature = "schemars")]
pub mod workspace_types;

pub use crate::configuration::{
    create_config, load_config, Configuration, ConfigurationError, RuleConfiguration, Rules,
};
pub use crate::matcher::{MatchOptions, Matcher, Pattern};

pub use crate::file_handlers::JsFormatSettings;
use crate::file_handlers::Language;
pub use crate::workspace::Workspace;

/// Exports only for this crate
pub(crate) use crate::configuration::{deserialize_set_of_strings, serialize_set_of_strings};

pub const VERSION: &str = match option_env!("ROME_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

/// This is the main entrypoint of the application.
pub struct App<'app> {
    /// A reference to the internal virtual file system
    pub fs: DynRef<'app, dyn FileSystem>,
    /// A reference to the internal workspace
    pub workspace: WorkspaceRef<'app>,
    /// A reference to the internal console, where its buffer will be used to write messages and
    /// errors
    pub console: DynRef<'app, dyn Console>,
}

#[derive(Serialize, Deserialize)]
/// Generic errors thrown during rome operations
pub enum RomeError {
    /// The project contains uncommitted changes
    DirtyWorkspace,
    /// The file does not exist in the [Workspace]
    NotFound,
    /// A file is not supported. It contains the language and path of the file
    /// Use this error if Rome is trying to process a file that Rome can't understand
    SourceFileNotSupported(Language, RomePath),
    /// The formatter encountered an error while formatting the file
    FormatError(FormatError),
    /// The file could not be formatted since it has syntax errors and `format_with_errors` is disabled
    FormatWithErrorsDisabled,
    /// The file could not be analyzed because a rule caused an error.
    RuleError(RuleError),
    /// Thrown when Rome can't read a generic directory
    CantReadDirectory(PathBuf),
    /// Thrown when Rome can't read a generic file
    CantReadFile(PathBuf),
    /// Error thrown when validating the configuration. Once deserialized, further checks have to be done.
    Configuration(ConfigurationError),
    /// Error thrown when Rome cannot rename a symbol.
    RenameError(RenameError),
    /// Error emitted by the underlying transport layer for a remote Workspace
    TransportError(TransportError),
    /// Emitted when the file is ignored and should not be processed
    FileIgnored(PathBuf),
    /// Emitted when a file could not be parsed because it's larger than the size limite
    FileTooLarge {
        path: PathBuf,
        size: usize,
        limit: usize,
    },
}

impl Debug for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Display for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RomeError::SourceFileNotSupported(language, path) => {
                if *language != Language::Unknown {
                    write!(
                        f,
                        "Rome doesn't support this feature for the language {language:?}"
                    )
                } else if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                    write!(
                        f,
                        "Rome could not determine the language for the file extension {ext:?}"
                    )
                } else {
                    write!(
                        f,
                        "Rome could not determine the language for the file {path:?} because it doesn't have a clear extension"
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
                    path.display()
                )
            }
            RomeError::CantReadFile(path) => {
                write!(
                    f,
                    "Rome couldn't read the following file, maybe for permissions reasons or it doesn't exists: {}",
                    path.display()
                )
            }

            RomeError::Configuration(error) => fmt::Display::fmt(error, f),
            RomeError::DirtyWorkspace => {
                write!(f, "Uncommitted changes in repository")
            }
            RomeError::RenameError(error) => match error {
                RenameError::CannotBeRenamed {
                    original_name,
                    new_name,
                } => {
                    write!(
                        f,
                        "encountered an error while renaming the symbol \"{}\" to \"{}\"",
                        original_name, new_name
                    )
                }
                RenameError::CannotFindDeclaration => {
                    write!(
                        f,
                        "encountered an error finding a declaration at the specified position"
                    )
                }
            },
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
                write!(f, "The file {} was ignored", path.display())
            }
            RomeError::FileTooLarge { path, size, limit } => {
                write!(f, "Size of {} is {} which exceeds the project maximum of {}. The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't.", path.display(), Bytes(*size), Bytes(*limit))
            }
        }
    }
}

impl Error for RomeError {}

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

impl Default for App<'static> {
    fn default() -> Self {
        Self::with_filesystem_and_console(
            DynRef::Owned(Box::new(OsFileSystem)),
            DynRef::Owned(Box::new(EnvConsole::default())),
        )
    }
}

impl<'app> App<'app> {
    /// Create a new instance of the app using the specified [FileSystem] and [Console] implementation
    pub fn with_filesystem_and_console(
        fs: DynRef<'app, dyn FileSystem>,
        console: DynRef<'app, dyn Console>,
    ) -> Self {
        Self::new(fs, console, WorkspaceRef::Owned(workspace::server()))
    }

    /// Create a new instance of the app using the specified [FileSystem], [Console] and [Workspace] implementation
    pub fn new(
        fs: DynRef<'app, dyn FileSystem>,
        console: DynRef<'app, dyn Console>,
        workspace: WorkspaceRef<'app>,
    ) -> Self {
        Self {
            fs,
            console,
            workspace,
        }
    }
}

pub enum WorkspaceRef<'app> {
    Owned(Box<dyn Workspace>),
    Borrowed(&'app dyn Workspace),
}

impl<'app> Deref for WorkspaceRef<'app> {
    type Target = dyn Workspace + 'app;

    fn deref(&self) -> &Self::Target {
        match self {
            WorkspaceRef::Owned(inner) => &**inner,
            WorkspaceRef::Borrowed(inner) => *inner,
        }
    }
}

/// Clone of [std::borrow::Cow] specialized for storing a trait object and
/// holding a mutable reference in the `Borrowed` variant instead of requiring
/// the inner type to implement [std::borrow::ToOwned]
pub enum DynRef<'app, T: ?Sized + 'app> {
    Owned(Box<T>),
    Borrowed(&'app mut T),
}

impl<'app, T: ?Sized + 'app> Deref for DynRef<'app, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            DynRef::Owned(inner) => inner,
            DynRef::Borrowed(inner) => inner,
        }
    }
}

impl<'app, T: ?Sized + 'app> DerefMut for DynRef<'app, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            DynRef::Owned(inner) => inner,
            DynRef::Borrowed(inner) => inner,
        }
    }
}
