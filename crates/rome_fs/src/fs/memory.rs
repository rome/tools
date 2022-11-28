use std::collections::hash_map::Entry;
use std::collections::{hash_map::IntoIter, HashMap};
use std::io;
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};
use std::str;
use std::sync::Arc;

use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex, RwLock};
use rome_diagnostics::Error;

use crate::fs::OpenOptions;
use crate::{FileSystem, RomePath, TraversalContext, TraversalScope};

use super::{BoxedTraversal, ErrorKind, File, FileSystemDiagnostic};

/// Fully in-memory file system, stores the content of all known files in a hashmap
pub struct MemoryFileSystem {
    files: AssertUnwindSafe<RwLock<HashMap<PathBuf, FileEntry>>>,
    errors: HashMap<PathBuf, ErrorEntry>,
    allow_write: bool,
}

impl Default for MemoryFileSystem {
    fn default() -> Self {
        Self {
            files: Default::default(),
            errors: Default::default(),
            allow_write: true,
        }
    }
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

/// Error entries are special file system entries that cause an error to be
/// emitted when they are reached through a filesystem traversal. This is
/// mainly useful as a mechanism to test the handling of filesystem error in
/// client code.
#[derive(Clone, Debug)]
pub enum ErrorEntry {
    UnknownFileType,
    DereferencedSymlink(PathBuf),
    InfiniteSymlinkExpansion(PathBuf),
}

impl MemoryFileSystem {
    /// Create a read-only instance of [MemoryFileSystem]
    ///
    /// This instance will disallow any modification through the [FileSystem]
    /// trait, but the content of the filesystem may still be modified using
    /// the methods on [MemoryFileSystem] itself.
    pub fn new_read_only() -> Self {
        Self {
            allow_write: false,
            ..Self::default()
        }
    }

    /// Create or update a file in the filesystem
    pub fn insert(&mut self, path: PathBuf, content: impl Into<Vec<u8>>) {
        let files = self.files.0.get_mut();
        files.insert(path, Arc::new(Mutex::new(content.into())));
    }

    /// Create or update an error in the filesystem
    pub fn insert_error(&mut self, path: PathBuf, kind: ErrorEntry) {
        self.errors.insert(path, kind);
    }

    /// Remove a file from the filesystem
    pub fn remove(&mut self, path: &Path) {
        self.files.0.write().remove(path);
    }

    pub fn files(self) -> IntoIter<PathBuf, FileEntry> {
        let files = self.files.0.into_inner();
        files.into_iter()
    }
}

impl FileSystem for MemoryFileSystem {
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
        if !self.allow_write
            && (options.create || options.create_new || options.truncate || options.write)
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cannot acquire write access to file in read-only filesystem",
            ));
        }

        let mut inner = if options.create || options.create_new {
            // Acquire write access to the files map if the file may need to be created
            let mut files = self.files.0.write();
            match files.entry(PathBuf::from(path)) {
                Entry::Vacant(entry) => {
                    // we create an empty file
                    let file: FileEntry = Arc::new(Mutex::new(vec![]));
                    let entry = entry.insert(file);
                    entry.lock_arc()
                }
                Entry::Occupied(entry) => {
                    if options.create {
                        // If `create` is true, truncate the file
                        let entry = entry.into_mut();
                        *entry = Arc::new(Mutex::new(vec![]));
                        entry.lock_arc()
                    } else {
                        // This branch can only be reached if `create_new` was true,
                        // we should return an error if the file already exists
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            format!("path {path:?} already exists in memory filesystem"),
                        ));
                    }
                }
            }
        } else {
            let files = self.files.0.read();
            let entry = files.get(path).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("path {path:?} does not exists in memory filesystem"),
                )
            })?;

            entry.lock_arc()
        };

        if options.truncate {
            // Clear the buffer if the file was open with `truncate`
            inner.clear();
        }

        Ok(Box::new(MemoryFile {
            inner,
            can_read: options.read,
            can_write: options.write,
        }))
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        func(&MemoryTraversalScope { fs: self })
    }
}

struct MemoryFile {
    inner: ArcMutexGuard<RawMutex, Vec<u8>>,
    can_read: bool,
    can_write: bool,
}

impl File for MemoryFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        if !self.can_read {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "this file wasn't open with read access",
            ));
        }

        // Verify the stored byte content is valid UTF-8
        let content = str::from_utf8(&self.inner)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        // Append the content of the file to the buffer
        buffer.push_str(content);
        Ok(())
    }

    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        if !self.can_write {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "this file wasn't open with write access",
            ));
        }

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
        {
            let files = &self.fs.files.0.read();
            for path in files.keys() {
                let should_process_file = if base.starts_with(".") || base.starts_with("./") {
                    // we simulate absolute paths, so we can correctly strips out the base path from the path
                    let absolute_base = PathBuf::from("/").join(&base);
                    let absolute_path = Path::new("/").join(path);
                    absolute_path.strip_prefix(&absolute_base).is_ok()
                } else {
                    path.strip_prefix(&base).is_ok()
                };

                if should_process_file {
                    let (file_id, _) = ctx.interner().intern_path(path.into());
                    let rome_path = RomePath::new(path, file_id);
                    if !ctx.can_handle(&rome_path) {
                        continue;
                    }
                    ctx.handle_file(path, file_id);
                }
            }
        }

        for (path, entry) in &self.fs.errors {
            if path.strip_prefix(&base).is_ok() {
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.to_string_lossy().to_string(),
                    error_kind: match entry {
                        ErrorEntry::UnknownFileType => ErrorKind::UnknownFileType,
                        ErrorEntry::DereferencedSymlink(path) => {
                            ErrorKind::DereferencedSymlink(path.to_string_lossy().to_string())
                        }
                        ErrorEntry::InfiniteSymlinkExpansion(path) => {
                            ErrorKind::InfiniteSymlinkExpansion(path.to_string_lossy().to_string())
                        }
                    },
                }));
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
    use rome_diagnostics::Error;

    use crate::{fs::FileSystemExt, OpenOptions};
    use crate::{FileSystem, MemoryFileSystem, PathInterner, RomePath, TraversalContext};
    use rome_diagnostics::location::FileId;

    #[test]
    fn fs_read_only() {
        let mut fs = MemoryFileSystem::new_read_only();

        let path = Path::new("file.js");
        fs.insert(path.into(), *b"content");

        assert!(fs.open(path).is_ok());

        match fs.create(path) {
            Ok(_) => panic!("fs.create() for a read-only filesystem should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            }
        }

        match fs.create_new(path) {
            Ok(_) => panic!("fs.create() for a read-only filesystem should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            }
        }

        match fs.open_with_options(path, OpenOptions::default().read(true).write(true)) {
            Ok(_) => panic!("fs.open_with_options(read + write) for a read-only filesystem should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            }
        }
    }

    #[test]
    fn file_read_write() {
        let mut fs = MemoryFileSystem::default();

        let path = Path::new("file.js");
        let content_1 = "content 1";
        let content_2 = "content 2";

        fs.insert(path.into(), content_1.as_bytes());

        let mut file = fs
            .open_with_options(path, OpenOptions::default().read(true).write(true))
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
    fn file_create() {
        let fs = MemoryFileSystem::default();

        let path = Path::new("file.js");
        let mut file = fs.create(path).expect("the file should not fail to open");

        file.set_content(b"content".as_slice())
            .expect("the file should be written without error");
    }

    #[test]
    fn file_create_truncate() {
        let mut fs = MemoryFileSystem::default();

        let path = Path::new("file.js");
        fs.insert(path.into(), b"content".as_slice());

        let file = fs.create(path).expect("the file should not fail to create");

        drop(file);

        let mut file = fs.open(path).expect("the file should not fail to open");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert!(
            buffer.is_empty(),
            "fs.create() should truncate the file content"
        );
    }

    #[test]
    fn file_create_new() {
        let fs = MemoryFileSystem::default();

        let path = Path::new("file.js");
        let content = "content";

        let mut file = fs
            .create_new(path)
            .expect("the file should not fail to create");

        file.set_content(content.as_bytes())
            .expect("the file should be written without error");

        drop(file);

        let mut file = fs.open(path).expect("the file should not fail to open");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert_eq!(buffer, content);
    }

    #[test]
    fn file_create_new_exists() {
        let mut fs = MemoryFileSystem::default();

        let path = Path::new("file.js");
        fs.insert(path.into(), b"content".as_slice());

        let result = fs.create_new(path);

        match result {
            Ok(_) => panic!("fs.create_new() for an existing file should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
            }
        }
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
            interner: PathInterner,
            visited: Mutex<Vec<PathBuf>>,
        }

        impl TraversalContext for TestContext {
            fn interner(&self) -> &PathInterner {
                &self.interner
            }

            fn push_diagnostic(&self, err: Error) {
                panic!("unexpected error {err:?}")
            }

            fn can_handle(&self, _: &RomePath) -> bool {
                true
            }

            fn handle_file(&self, path: &Path, _: FileId) {
                self.visited.lock().push(path.into())
            }
        }

        let (interner, _) = PathInterner::new();
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
