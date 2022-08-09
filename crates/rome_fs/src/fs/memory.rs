use std::collections::hash_map::IntoIter;
use std::{
    collections::HashMap,
    io,
    panic::AssertUnwindSafe,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};

use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex, RwLock};

use crate::fs::{FileSystemExt, OpenOptions};
use crate::{FileSystem, TraversalContext, TraversalScope};

use super::{BoxedTraversal, File};

/// Fully in-memory file system, stores the content of all known files in a hashmap
#[derive(Default)]
pub struct MemoryFileSystem {
    files: AssertUnwindSafe<RwLock<HashMap<PathBuf, FileEntry>>>,
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
type FileEntry = Arc<Mutex<Vec<u8>>>;

impl MemoryFileSystem {
    /// Create or update a file in the filesystem
    pub fn insert(&mut self, path: PathBuf, content: impl Into<Vec<u8>>) {
        let files = &mut self.files.0.write();
        files.insert(path, Arc::new(Mutex::new(content.into())));
    }

    pub fn files(self) -> IntoIter<PathBuf, FileEntry> {
        let files = self.files.0.into_inner();
        let iter = files.into_iter();
        iter
    }
}

impl FileSystem for MemoryFileSystem {
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
        if options.read && options.write {
            self.open(path)
        } else if options.create_new || options.write {
            self.create(path)
        } else {
            unimplemented!("the set of open options provided don't match any case")
        }
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        func(&MemoryTraversalScope { fs: self })
    }
}

impl FileSystemExt for MemoryFileSystem {
    fn create(&self, path: &Path) -> io::Result<Box<dyn File>> {
        let files = &mut self.files.0.write();
        // we create an empty file
        let file: FileEntry = Arc::new(Mutex::new(vec![]));
        let path = PathBuf::from(path);
        files.insert(path, file.clone());
        let inner = file.lock_arc();
        Ok(Box::new(MemoryFile { inner }))
    }

    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        let files = &self.files.0.read();
        let entry = files.get(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("path {path:?} does not exists in memory filesystem"),
            )
        })?;

        let lock = entry.lock_arc();

        Ok(Box::new(MemoryFile { inner: lock }))
    }

    fn read(&self, path: &Path) -> io::Result<Box<dyn File>> {
        let files = &self.files.0.read();
        let entry = files.get(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("path {path:?} does not exists in memory filesystem"),
            )
        })?;

        let lock = entry.lock_arc();

        Ok(Box::new(MemoryFile { inner: lock }))
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
        let files = &self.fs.files.0.read();
        for path in files.keys() {
            if path.strip_prefix(&base).is_ok() {
                let file_id = ctx.interner().intern_path(path.into());
                ctx.handle_file(path, file_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io,
        mem::swap,
        path::{Path, PathBuf},
    };

    use parking_lot::Mutex;

    use crate::fs::FileSystemExt;
    use crate::{
        interner::FileId, AtomicInterner, FileSystem, MemoryFileSystem, PathInterner, RomePath,
        TraversalContext,
    };

    #[test]
    fn file_read_write() {
        let mut fs = MemoryFileSystem::default();

        let path = Path::new("file.js");
        let content_1 = "content 1";
        let content_2 = "content 2";

        fs.insert(path.into(), content_1.as_bytes());

        let mut file = fs
            .open(path)
            .expect("the file should exist in the memory file system");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert_eq!(buffer, content_1);

        file.set_content(content_2.as_bytes())
            .expect("the file should be written without error");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert_eq!(buffer, content_2);
    }

    #[test]
    fn missing_file() {
        let fs = MemoryFileSystem::default();

        let result = fs.open(Path::new("non_existing"));

        match result {
            Ok(_) => panic!("opening a non-existing file should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::NotFound);
            }
        }
    }

    #[test]
    fn traversal() {
        let mut fs = MemoryFileSystem::default();

        fs.insert(PathBuf::from("dir1/file1"), "dir1/file1".as_bytes());
        fs.insert(PathBuf::from("dir1/file2"), "dir1/file1".as_bytes());
        fs.insert(PathBuf::from("dir2/file1"), "dir2/file1".as_bytes());
        fs.insert(PathBuf::from("dir2/file2"), "dir2/file1".as_bytes());

        struct TestContext {
            interner: AtomicInterner,
            visited: Mutex<Vec<PathBuf>>,
        }

        impl TraversalContext for TestContext {
            fn interner(&self) -> &dyn PathInterner {
                &self.interner
            }

            fn push_diagnostic(&self, file_id: FileId, code: &'static str, message: String) {
                panic!("unexpected error {code:?} in file {file_id}: {message}")
            }

            fn can_handle(&self, _: &RomePath) -> bool {
                true
            }

            fn handle_file(&self, path: &Path, _: FileId) {
                self.visited.lock().push(path.into())
            }
        }

        let (interner, _) = AtomicInterner::new();
        let mut ctx = TestContext {
            interner,
            visited: Mutex::default(),
        };

        // Traverse a directory
        fs.traversal(Box::new(|scope| {
            scope.spawn(&ctx, PathBuf::from("dir1"));
        }));

        let mut visited = Vec::new();
        swap(&mut visited, ctx.visited.get_mut());

        assert_eq!(visited.len(), 2);
        assert!(visited.contains(&PathBuf::from("dir1/file1")));
        assert!(visited.contains(&PathBuf::from("dir1/file2")));

        // Traverse a single file
        fs.traversal(Box::new(|scope| {
            scope.spawn(&ctx, PathBuf::from("dir2/file2"));
        }));

        let mut visited = Vec::new();
        swap(&mut visited, ctx.visited.get_mut());

        assert_eq!(visited.len(), 1);
        assert!(visited.contains(&PathBuf::from("dir2/file2")));
    }
}
