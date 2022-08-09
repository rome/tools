use rome_console::{Console, EnvConsole};
use rome_formatter::FormatError;
use rome_fs::{FileSystem, OsFileSystem, RomePath};
use rome_js_analyze::utils::rename::RenameError;
use rome_js_analyze::RuleError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

pub mod configuration;
mod file_handlers;
pub mod settings;
pub mod workspace;

pub use crate::configuration::{
    create_config, load_config, Configuration, ConfigurationError, RuleConfiguration, Rules,
};
pub use crate::file_handlers::JsFormatSettings;
pub use crate::workspace::Workspace;

/// This is the main entrypoint of the application.
pub struct App<'app> {
    /// A reference to the internal virtual file system
    pub fs: DynRef<'app, dyn FileSystem>,
    /// A reference to the internal workspace
    pub workspace: DynRef<'app, dyn Workspace>,
    /// A reference to the internal console, where its buffer will be used to write messages and
    /// errors
    pub console: DynRef<'app, dyn Console>,
}

/// Generic errors thrown during rome operations
pub enum RomeError {
    /// The project contains uncommitted changes
    DirtyWorkspace,
    /// The file does not exist in the [Workspace]
    NotFound,
    /// A file is not supported. It contains the extension of the file
    /// Use this error if Rome is trying to process a file that Rome can't understand
    SourceFileNotSupported(RomePath),
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
}

impl Debug for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for RomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RomeError::SourceFileNotSupported(path) => {
                let ext = path.extension().and_then(|ext| ext.to_str());

                if let Some(ext) = ext {
                    write!(f, "Rome doesn't support the file extension {ext:?} yet")
                } else {
                    write!(
                        f,
                        "Rome can't process the file because it doesn't have a clear extension"
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

            RomeError::Configuration(error) => std::fmt::Display::fmt(error, f),
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
        }
    }
}

impl Error for RomeError {}

impl From<FormatError> for RomeError {
    fn from(err: FormatError) -> Self {
        Self::FormatError(err)
    }
}

impl<'app> App<'app> {
    /// Create a new instance of the app using the [OsFileSystem] and [EnvConsole]
    pub fn from_env(no_colors: bool) -> Self {
        Self::with_filesystem_and_console(
            DynRef::Owned(Box::new(OsFileSystem)),
            DynRef::Owned(Box::new(EnvConsole::new(no_colors))),
        )
    }
}

impl<'app> App<'app> {
    /// Create a new instance of the app using the specified [FileSystem] and [Console] implementation
    pub fn with_filesystem_and_console(
        fs: DynRef<'app, dyn FileSystem>,
        console: DynRef<'app, dyn Console>,
    ) -> Self {
        Self {
            fs,
            console,
            workspace: DynRef::Owned(workspace::server()),
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
