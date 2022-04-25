//! Implementation of the [FileSystem] and related traits for the underlying OS filesystem
use std::{
    ffi::OsStr,
    fs,
    io::{self, Read, Seek, SeekFrom, Write},
    mem,
    path::{Path, PathBuf},
};

use rayon::{scope, Scope};

use crate::{
    fs::{TraversalContext, TraversalScope},
    interner::FileId,
    FileSystem, RomePath,
};

use super::{BoxedTraversal, File};

/// Implementation of [FileSystem] that directly calls through to the underlying OS
pub struct OsFileSystem;

impl FileSystem for OsFileSystem {
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        tracing::debug_span!("OsFileSystem::open", path = ?path).in_scope(
            move || -> io::Result<Box<dyn File>> {
                Ok(Box::new(OsFile {
                    inner: fs::File::options().read(true).write(true).open(path)?,
                }))
            },
        )
    }

    fn traversal(&self, func: BoxedTraversal) {
        OsTraversalScope::with(move |scope| {
            func(scope);
        })
    }
}

struct OsFile {
    inner: fs::File,
}

impl File for OsFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        tracing::debug_span!("OsFile::read_to_string").in_scope(move || {
            // Reset the cursor to the starting position
            self.inner.seek(SeekFrom::Start(0))?;
            // Read the file content
            self.inner.read_to_string(buffer)?;
            Ok(())
        })
    }

    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        tracing::debug_span!("OsFile::set_content").in_scope(move || {
            // Truncate the file
            self.inner.set_len(0)?;
            // Reset the cursor to the starting position
            self.inner.seek(SeekFrom::Start(0))?;
            // Write the byte slice
            self.inner.write_all(content)?;
            Ok(())
        })
    }
}

#[repr(transparent)]
pub struct OsTraversalScope<'scope> {
    scope: Scope<'scope>,
}

impl<'scope> OsTraversalScope<'scope> {
    pub(crate) fn with<F>(func: F)
    where
        F: FnOnce(&Self) + Send,
    {
        scope(move |scope| func(Self::from_rayon(scope)))
    }

    fn from_rayon<'a>(scope: &'a Scope<'scope>) -> &'a Self {
        // SAFETY: transmuting from Scope to OsTraversalScope is safe since
        // OsTraversalScope has the `repr(transparent)` attribute that
        // guarantees its layout is the same as Scope
        unsafe { mem::transmute(scope) }
    }
}

impl<'scope> TraversalScope<'scope> for OsTraversalScope<'scope> {
    fn spawn(&self, ctx: &'scope dyn TraversalContext, path: PathBuf) {
        let file_id = ctx.interner().intern_path(path.clone());

        let file_type = match path.metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                ctx.push_diagnostic(file_id, "IO", err.to_string());
                return;
            }
        };

        if file_type.is_file() {
            self.scope.spawn(move |_| {
                ctx.handle_file(&path, file_id);
            });
            return;
        }

        if file_type.is_dir() {
            self.scope.spawn(move |scope| {
                handle_dir(scope, ctx, &path, file_id);
            });
            return;
        }

        ctx.push_diagnostic(file_id, "IO", "unhandled file type".into());
    }
}

/// Default list of ignored directories, in the future will be supplanted by
/// detecting and parsing .ignore files
const DEFAULT_IGNORE: &[&str] = &[".git"];

/// Traverse a single directory, scheduling any file to execute the context
/// handler and sub-directories for subsequent traversal
use ignore::{Walk, WalkBuilder};

fn handle_dir<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    path: &Path,
    file_id: FileId,
) {
    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
        if DEFAULT_IGNORE.contains(&file_name) {
            return;
        }
    }

    let iter = if ctx.use_gitignore() {
        Walk::new(path)
    } else {
        WalkBuilder::new(path)
            .ignore(false)
            .git_ignore(false)
            .git_exclude(false)
            .build()
    };

    for entry in iter {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                ctx.push_diagnostic(file_id, "IO", err.to_string());
                continue;
            }
        };

        let entry_path = entry.path().to_path_buf();

        if entry_path == path {
            continue;
        }

        let file_id = ctx.interner().intern_path(entry_path.clone());

        let file_type = match entry.file_type() {
            Some(file_type) => file_type,
            None => {
                ctx.push_diagnostic(file_id, "IO", "file type does not exist".to_string());
                continue;
            }
        };

        if file_type.is_dir() {
            scope.spawn(move |scope| {
                handle_dir(scope, ctx, &entry_path, file_id);
            });
            continue;
        }

        if file_type.is_file() {
            // Performing this check here lets us skip scheduling unsupported
            // files entirely as well as silently ignore unsupported files when
            // doing a directory traversal but printing an error message if the
            // user explicitly requests an unsupported file to be formatted
            if !ctx.can_handle(&RomePath::new(&entry_path)) {
                continue;
            }

            scope.spawn(move |_| {
                ctx.handle_file(&entry_path, file_id);
            });
            continue;
        }

        ctx.push_diagnostic(file_id, "IO", "unhandled file type".into());
    }
}
