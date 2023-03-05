//! Implementation of the [FileSystem] and related traits for the underlying OS filesystem
use super::{BoxedTraversal, ErrorKind, File, FileSystemDiagnostic};
use crate::fs::OpenOptions;
use crate::{
    fs::{TraversalContext, TraversalScope},
    FileSystem, RomePath,
};
use rayon::{scope, Scope};
use rome_diagnostics::{adapters::IoError, DiagnosticExt, Error};
use std::fs::DirEntry;
use std::{
    ffi::OsStr,
    fs,
    io::{self, ErrorKind as IoErrorKind, Read, Seek, SeekFrom, Write},
    mem,
    path::{Path, PathBuf},
};

/// Implementation of [FileSystem] that directly calls through to the underlying OS
pub struct OsFileSystem;

impl FileSystem for OsFileSystem {
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
        tracing::debug_span!("OsFileSystem::open_with_options", path = ?path, options = ?options)
            .in_scope(move || -> io::Result<Box<dyn File>> {
                let mut fs_options = fs::File::options();
                Ok(Box::new(OsFile {
                    inner: options.into_fs_options(&mut fs_options).open(path)?,
                }))
            })
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
        let file_type = match path.metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                ctx.push_diagnostic(
                    IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                );
                return;
            }
        };

        if file_type.is_symlink() {
            tracing::info!("Translating symlink: {:?}", path);
            let path = match fs::read_link(&path) {
                Ok(path) => path,
                Err(err) => {
                    ctx.push_diagnostic(
                        IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                    );
                    return;
                }
            };

            if let Err(err) = fs::symlink_metadata(&path) {
                if err.kind() == IoErrorKind::NotFound {
                    let path = path.to_string_lossy().to_string();
                    ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                        path: path.clone(),
                        error_kind: ErrorKind::DereferencedSymlink(path),
                    }));
                } else {
                    ctx.push_diagnostic(
                        IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                    );
                }
                return;
            };

            return self.spawn(ctx, path);
        };

        let _ = ctx.interner().intern_path(path.clone());

        if file_type.is_file() {
            self.scope.spawn(move |_| {
                ctx.handle_file(&path);
            });
            return;
        }

        if file_type.is_dir() {
            self.scope.spawn(move |scope| {
                handle_dir(scope, ctx, &path, None);
            });
            return;
        }

        ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
            path: path.to_string_lossy().to_string(),
            error_kind: ErrorKind::from(file_type),
        }));
    }
}

/// Default list of ignored directories, in the future will be supplanted by
/// detecting and parsing .ignore files
/// TODO: add support for ignore files in Rome
const DEFAULT_IGNORE: &[&str; 5] = &[".git", ".svn", ".hg", ".yarn", "node_modules"];

/// Traverse a single directory
fn handle_dir<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    path: &Path,
    origin_path: Option<PathBuf>,
) {
    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
        if DEFAULT_IGNORE.contains(&file_name) {
            return;
        }
    }

    let iter = match fs::read_dir(path) {
        Ok(iter) => iter,
        Err(err) => {
            ctx.push_diagnostic(IoError::from(err).with_file_path(path.display().to_string()));
            return;
        }
    };

    for entry in iter {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(path.display().to_string()));
                continue;
            }
        };

        handle_dir_entry(scope, ctx, entry, origin_path.clone());
    }
}

/// Traverse a single directory entry, scheduling any file to execute the context
/// handler and sub-directories for subsequent traversal
fn handle_dir_entry<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    entry: DirEntry,
    mut origin_path: Option<PathBuf>,
) {
    let mut path = entry.path();

    let mut file_type = match entry.file_type() {
        Ok(file_type) => file_type,
        Err(err) => {
            ctx.push_diagnostic(
                IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
            );
            return;
        }
    };

    if file_type.is_symlink() {
        tracing::info!("Translating symlink: {:?}", path);
        let target_path = match fs::read_link(&path) {
            Ok(path) => path,
            Err(err) => {
                ctx.push_diagnostic(
                    IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                );
                return;
            }
        };

        file_type = match path.metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                if err.kind() == IoErrorKind::NotFound {
                    let path = path.to_string_lossy().to_string();
                    ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                        path: path.clone(),
                        error_kind: ErrorKind::DereferencedSymlink(path),
                    }));
                } else {
                    ctx.push_diagnostic(
                        IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                    );
                }
                return;
            }
        };

        if file_type.is_dir() {
            origin_path = Some(path);
        }

        path = target_path;
    };

    let inserted = ctx.interner().intern_path(path.clone());

    // Determine whether an equivalent path already exists
    if !inserted {
        let path = path.to_string_lossy().to_string();
        ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
            path: path.clone(),
            error_kind: ErrorKind::InfiniteSymlinkExpansion(path),
        }));
        return;
    }

    if file_type.is_dir() {
        scope.spawn(move |scope| {
            handle_dir(scope, ctx, &path, origin_path);
        });
        return;
    }

    if file_type.is_file() {
        if matches!(
            path.file_name().and_then(OsStr::to_str),
            Some("package.json" | "package-lock.json" | "tsconfig.json" | "jsconfig.json")
        ) {
            return;
        }

        let rome_path = if let Some(origin_path) = origin_path {
            if let Some(file_name) = path.file_name() {
                RomePath::new(origin_path.join(file_name))
            } else {
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.to_string_lossy().to_string(),
                    error_kind: ErrorKind::UnknownFileType,
                }));
                return;
            }
        } else {
            RomePath::new(&path)
        };

        // Performing this check here let's us skip skip unsupported
        // files entirely, as well as silently ignore unsupported files when
        // doing a directory traversal, but printing an error message if the
        // user explicitly requests an unsupported file to be handled
        if !ctx.can_handle(&rome_path) {
            return;
        }

        scope.spawn(move |_| {
            ctx.handle_file(&path);
        });
        return;
    }

    ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
        path: path.to_string_lossy().to_string(),
        error_kind: ErrorKind::from(file_type),
    }));
}

impl From<fs::FileType> for ErrorKind {
    fn from(_: fs::FileType) -> Self {
        Self::UnknownFileType
    }
}
