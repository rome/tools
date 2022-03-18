use std::{
    io,
    panic::RefUnwindSafe,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{interner::FileId, PathInterner, RomePath};

mod memory;
mod os;

pub use memory::MemoryFileSystem;
pub use os::OsFileSystem;

pub trait FileSystem: Sync + RefUnwindSafe {
    /// Open a handle to the file at `path`
    ///
    /// Currently this locks down the file for both reading and writing
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>>;

    /// Initiate a traversal of the filesystem
    ///
    /// This method creates a new "traversal scope" that can be used to
    /// efficiently batch many filesystem read operations
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>);
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
    fn interner(&self) -> &dyn PathInterner;

    /// Called by the traversal process to emit an error diagnostic associated
    /// with a particular file ID when an IO error happens
    fn push_diagnostic(&self, file_id: FileId, code: &'static str, message: String);

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
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        T::open(self, path)
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        T::traversal(self, func)
    }
}
