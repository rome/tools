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
    /// efficient batch many filesystem read operations
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>);
}

pub trait File {
    /// Read the content of the file into `buffer`
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()>;

    /// Overwrite the content of the file with the provided bytes
    fn set_content(&mut self, content: &[u8]) -> io::Result<()>;
}

type BoxedTraversal<'fs, 'scope> = Box<dyn FnOnce(&dyn TraversalScope<'scope>) + Send + 'fs>;

pub trait TraversalScope<'scope> {
    /// Spawn a new filesystem read task
    ///
    /// If the provided path exists and is a file, the [`handle_file`](TraversalContext::handle_file)
    /// method of the provided [TraversalContext] will be called. If its a
    /// directory, it will be recursively traversed and all the files the
    /// [`can_handle`](TraversalContext::can_handle) method of the context
    /// returns true for will be handled as well
    fn spawn(&self, context: &'scope dyn TraversalContext, path: PathBuf);
}

pub trait TraversalContext: Sync {
    fn interner(&self) -> &dyn PathInterner;

    fn push_diagnostic(&self, file_id: FileId, code: &'static str, message: String);
    fn can_handle(&self, path: &RomePath) -> bool;
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
