use std::{
    collections::HashMap,
    io,
    panic::AssertUnwindSafe,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};

use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex};

use crate::{FileSystem, TraversalContext, TraversalScope};

use super::{BoxedTraversal, File};

/// Fully in-memory file system, stores the content of all known files in a hashmap
#[derive(Default)]
pub struct MemoryFileSystem {
    files: HashMap<PathBuf, FileEntry>,
}

/// This is what's actually being stored for each file in the filesystem
///
/// To break it down:
/// - `Vec<u8>` is the byte buffer holding the content of the file
/// - `Mutex` lets it safely be read an written concurrently from multiple
///   threads ([FileSystem] is required to be [Sync])
/// - `Arc` allows [MemoryFile] handles to outlive references to the filesystem
///   itself (since [FileSystem::open] returns an owned value)
/// - `AssertUnwindSafe` tells the type system this value can safely be
///   accessed again after being recovered from a panic (using `catch_unwind`),
///   which means the filesystem guarantees a file will never get into an
///   inconsistent state if a thread panics while having a handle open (a read
///   or write either happens or not, but will never panic halfway through)
type FileEntry = AssertUnwindSafe<Arc<Mutex<Vec<u8>>>>;

impl MemoryFileSystem {
    /// Create or update a file in the filesystem
    pub fn insert(&mut self, path: PathBuf, content: impl Into<Vec<u8>>) {
        self.files
            .insert(path, AssertUnwindSafe(Arc::new(Mutex::new(content.into()))));
    }
}

impl FileSystem for MemoryFileSystem {
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        let entry = self.files.get(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("path {path:?} does not exists in memory filesystem"),
            )
        })?;

        let entry = entry.0.clone();
        let lock = entry.lock_arc();

        Ok(Box::new(MemoryFile { inner: lock }))
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        func(&MemoryTraversalScope { fs: self })
    }
}

struct MemoryFile {
    inner: ArcMutexGuard<RawMutex, Vec<u8>>,
}

impl File for MemoryFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        // Verify the stored byte content is valid UTF-8
        let content = str::from_utf8(&self.inner)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        // Append the content of the file to the buffer
        buffer.push_str(content);
        Ok(())
    }

    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        // Resize the memory buffer to fit the new content
        self.inner.resize(content.len(), 0);
        // Copy the new content into the memory buffer
        self.inner.copy_from_slice(content);
        Ok(())
    }
}

pub struct MemoryTraversalScope<'scope> {
    fs: &'scope MemoryFileSystem,
}

impl<'scope> TraversalScope<'scope> for MemoryTraversalScope<'scope> {
    fn spawn(&self, ctx: &'scope dyn TraversalContext, base: PathBuf) {
        // Traversal is implemented by iterating on all keys, and matching on
        // those that are prefixed with the provided `base` path
        for path in self.fs.files.keys() {
            if path.strip_prefix(&base).is_ok() {
                let file_id = ctx.interner().intern_path(path.into());
                ctx.handle_file(path, file_id);
            }
        }
    }
}
