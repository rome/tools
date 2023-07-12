use crate::{PathInterner, RomePath};
pub use memory::{ErrorEntry, MemoryFileSystem};
pub use os::OsFileSystem;
use rome_diagnostics::{console, Advices, Diagnostic, LogCategory, Visit};
use rome_diagnostics::{Error, Severity};
use serde::{Deserialize, Serialize};
use std::io;
use std::panic::RefUnwindSafe;
use std::path::{Path, PathBuf};
use tracing::{error, info};

mod memory;
mod os;

pub const CONFIG_NAME: &str = "rome.json";

pub trait FileSystem: Send + Sync + RefUnwindSafe {
    /// It opens a file with the given set of options
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>>;

    /// Initiate a traversal of the filesystem
    ///
    /// This method creates a new "traversal scope" that can be used to
    /// efficiently batch many filesystem read operations
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>);

    /// Returns the name of the main configuration file
    fn config_name(&self) -> &str {
        CONFIG_NAME
    }

    /// Return the path to the working directory
    fn working_directory(&self) -> Option<PathBuf>;

    /// Checks if the given path exists in the file system
    fn path_exists(&self, path: &Path) -> bool;

    /// Method that takes a path to a folder `file_path`, and a `file_name`. It attempts to find
    /// and read the file from that folder and if not found, it reads the parent directories recursively
    /// until:
    /// - the file is found, then it reads and return its contents
    /// - the file is not found
    ///
    /// If `should_error_if_file_not_found` it `true`, it returns an error.
    ///
    /// ## Errors
    ///
    /// - The file can't be read
    ///
    fn auto_search(
        &self,
        mut file_path: PathBuf,
        file_name: &str,
        should_error_if_file_not_found: bool,
    ) -> Result<Option<AutoSearchResult>, FileSystemDiagnostic> {
        let mut from_parent = false;
        let mut file_directory_path = file_path.join(file_name);
        loop {
            let options = OpenOptions::default().read(true);
            let file = self.open_with_options(&file_directory_path, options);
            return match file {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)
                        .map_err(|_| FileSystemDiagnostic {
                            path: file_directory_path.display().to_string(),
                            severity: Severity::Error,
                            error_kind: FsErrorKind::CantReadFile(
                                file_directory_path.display().to_string(),
                            ),
                        })?;

                    if from_parent {
                        info!(
                        "Rome auto discovered the file at following path that wasn't in the working directory: {}",
                        file_path.display()
                    );
                    }

                    return Ok(Some(AutoSearchResult {
                        content: buffer,
                        file_path: file_directory_path,
                        directory_path: file_path,
                    }));
                }
                Err(err) => {
                    // base paths from users are not eligible for auto discovery
                    if !should_error_if_file_not_found {
                        let parent_directory = if let Some(path) = file_path.parent() {
                            if path.is_dir() {
                                Some(PathBuf::from(path))
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        if let Some(parent_directory) = parent_directory {
                            file_path = parent_directory;
                            file_directory_path = file_path.join(file_name);
                            from_parent = true;
                            continue;
                        }
                    }
                    // We skip the error when the configuration file is not found.
                    // Not having a configuration file is only an error when the `base_path` is
                    // set to `BasePath::FromUser`.
                    if should_error_if_file_not_found || err.kind() != io::ErrorKind::NotFound {
                        return Err(FileSystemDiagnostic {
                            path: file_directory_path.display().to_string(),
                            severity: Severity::Error,
                            error_kind: FsErrorKind::CantReadFile(
                                file_directory_path.display().to_string(),
                            ),
                        });
                    }
                    error!(
                        "Could not read the file from {:?}, reason:\n {}",
                        file_directory_path.display(),
                        err
                    );
                    Ok(None)
                }
            };
        }
    }
}

/// Result of the auto search
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AutoSearchResult {
    /// The content of the file
    pub content: String,
    /// The path of the directory where the file was found
    pub directory_path: PathBuf,
    /// The path of the file found
    pub file_path: PathBuf,
}

pub trait File {
    /// Read the content of the file into `buffer`
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()>;

    /// Overwrite the content of the file with the provided bytes
    ///
    /// This will write to the associated memory buffer, as well as flush the
    /// new content to the disk if this is a physical file
    fn set_content(&mut self, content: &[u8]) -> io::Result<()>;

    /// Returns the version of the current file
    fn file_version(&self) -> i32;
}

/// This struct is a "mirror" of [std::fs::FileOptions].
/// Refer to their documentation for more details
#[derive(Default, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    pub fn read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }
    pub fn write(mut self, write: bool) -> Self {
        self.write = write;
        self
    }
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
    pub fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }
    pub fn create_new(mut self, create_new: bool) -> Self {
        self.create_new = create_new;
        self
    }

    pub fn into_fs_options(self, options: &mut std::fs::OpenOptions) -> &mut std::fs::OpenOptions {
        options
            .read(self.read)
            .write(self.write)
            .truncate(self.truncate)
            .create(self.create)
            .create_new(self.create_new)
    }
}

/// Trait that contains additional methods to work with [FileSystem]
pub trait FileSystemExt: FileSystem {
    /// Open a file with the `read` option
    ///
    /// Equivalent to [std::fs::File::open]
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(path, OpenOptions::default().read(true))
    }

    /// Open a file with the `write` and `create` options
    ///
    /// Equivalent to [std::fs::File::create]
    fn create(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(
            path,
            OpenOptions::default()
                .write(true)
                .create(true)
                .truncate(true),
        )
    }

    /// Opens a file with the `read`, `write` and `create_new` options
    ///
    /// Equivalent to [std::fs::File::create_new]
    fn create_new(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(
            path,
            OpenOptions::default()
                .read(true)
                .write(true)
                .create_new(true),
        )
    }
}

impl<T: ?Sized> FileSystemExt for T where T: FileSystem {}

type BoxedTraversal<'fs, 'scope> = Box<dyn FnOnce(&dyn TraversalScope<'scope>) + Send + 'fs>;

pub trait TraversalScope<'scope> {
    /// Spawn a new filesystem read task
    ///
    /// If the provided path exists and is a file, then the [`handle_file`](TraversalContext::handle_file)
    /// method of the provided [TraversalContext] will be called. If it's a
    /// directory, it will be recursively traversed and all the files the
    /// [`can_handle`](TraversalContext::can_handle) method of the context
    /// returns true for will be handled as well
    fn spawn(&self, context: &'scope dyn TraversalContext, path: PathBuf);
}

pub trait TraversalContext: Sync {
    /// Provides the traversal scope with an instance of [PathInterner], used
    /// to emit diagnostics for IO errors that may happen in the traversal process
    fn interner(&self) -> &PathInterner;

    /// Called by the traversal process to emit an error diagnostic associated
    /// with a particular file ID when an IO error happens
    fn push_diagnostic(&self, error: Error);

    /// Checks if the traversal context can handle a particular path, used as
    /// an optimization to bail out of scheduling a file handler if it wouldn't
    /// be able to process the file anyway
    fn can_handle(&self, path: &RomePath) -> bool;

    /// This method will be called by the traversal for each file it finds
    /// where [TraversalContext::can_handle] returned true
    fn handle_file(&self, path: &Path);
}

#[derive(Debug, Diagnostic, Deserialize, Serialize)]
#[diagnostic(category = "internalError/fs")]
pub struct FileSystemDiagnostic {
    #[severity]
    pub severity: Severity,
    #[location(resource)]
    pub path: String,
    #[message]
    #[description]
    #[advice]
    pub error_kind: FsErrorKind,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FsErrorKind {
    /// File not found
    CantReadFile(String),
    /// Unknown file type
    UnknownFileType,
    /// Dereferenced (broken) symbolic link
    DereferencedSymlink(String),
    /// Symbolic link cycle or symbolic link infinite expansion
    InfiniteSymlinkExpansion(String),

    CantAccessToFileSystem,
}

impl console::fmt::Display for FsErrorKind {
    fn fmt(&self, fmt: &mut console::fmt::Formatter) -> io::Result<()> {
        match self {
            FsErrorKind::CantReadFile(_) => fmt.write_str("Rome couldn't read the file"),
            FsErrorKind::UnknownFileType => fmt.write_str("Unknown file type"),
            FsErrorKind::DereferencedSymlink(_) => fmt.write_str("Dereferenced symlink"),
            FsErrorKind::InfiniteSymlinkExpansion(_) => fmt.write_str("Infinite symlink expansion"),
            FsErrorKind::CantAccessToFileSystem => fmt.write_str("Couldn't access to file system"),
        }
    }
}

impl std::fmt::Display for FsErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsErrorKind::CantReadFile(_) => fmt.write_str("Rome couldn't read the file"),
            FsErrorKind::UnknownFileType => write!(fmt, "Unknown file type"),
            FsErrorKind::DereferencedSymlink(_) => write!(fmt, "Dereferenced symlink"),
            FsErrorKind::InfiniteSymlinkExpansion(_) => write!(fmt, "Infinite symlink expansion"),
            FsErrorKind::CantAccessToFileSystem => fmt.write_str("Couldn't access to file system"),
        }
    }
}

impl Advices for FsErrorKind {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        match self {
			FsErrorKind::CantReadFile(path) => visitor.record_log(
		LogCategory::Error,
			&format!("Rome couldn't read the following file, maybe for permissions reasons or it doesn't exists: {}", path)
			),

            FsErrorKind::UnknownFileType => visitor.record_log(
                LogCategory::Info,
                &"Rome encountered a file system entry that's neither a file, directory or symbolic link",
            ),
            FsErrorKind::DereferencedSymlink(path) => visitor.record_log(
                LogCategory::Info,
                &format!("Rome encountered a file system entry that is a broken symbolic link: {}", path),
            ),
            FsErrorKind::InfiniteSymlinkExpansion(path) => visitor.record_log(
                LogCategory::Error,
                &format!("Rome encountered a file system entry that leads to an infinite symbolic link expansion, causing an infinite cycle: {}", path),
            ),
            FsErrorKind::CantAccessToFileSystem => visitor.record_log(
                LogCategory::Error,
                &format!("Some error"),
            ),

        }
    }
}
