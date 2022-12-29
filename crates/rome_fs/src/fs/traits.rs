use super::{OpenOptions, CONFIG_NAME};
use crate::{PathInterner, RomePath};

use rome_diagnostics::location::FileId;
use std::io;
use std::panic::RefUnwindSafe;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rome_diagnostics::Error;

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
}

pub trait File {
    /// Read the content of the file into `buffer`
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()>;

    /// Overwrite the content of the file with the provided bytes
    ///
    /// This will write to the associated memory buffer, as well as flush the
    /// new content to the disk if this is a physical file
    fn set_content(&mut self, content: &[u8]) -> io::Result<()>;
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

pub type BoxedTraversal<'fs, 'scope> = Box<dyn FnOnce(&dyn TraversalScope<'scope>) + Send + 'fs>;

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
    fn handle_file(&self, path: &Path, file_id: FileId);
}

impl<T> FileSystem for Arc<T>
where
    T: FileSystem + Send,
{
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
        T::open_with_options(self, path, options)
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        T::traversal(self, func)
    }
}
