use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ffi::{OsStr, OsString},
    fmt::Display,
    fs::{self, read_dir},
    io::{self, Read, Seek, SeekFrom, Write},
    ops::Range,
    panic::catch_unwind,
    path::{Path, PathBuf},
    process,
    str::Utf8Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Sender},
    },
    time::Instant,
};

use rayon::{self, scope, Scope};
use rome_core::App;
use rome_formatter::{FormatOptions, IndentStyle};
use rome_path::RomePath;
use rslint_errors::{
    file::{FileId, Files, SimpleFile},
    Diagnostic, Emitter,
};
use rslint_parser::{parse, SourceType};

use crate::CliSession;

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) {
    let mut options = FormatOptions::default();

    let size = session
        .args
        .opt_value_from_str("--indent-size")
        .expect("failed to parse indent-size argument");

    let style = session
        .args
        .opt_value_from_str("--indent-style")
        .expect("failed to parse indent-style argument");

    match style {
        Some(IndentStyle::Tab) => {
            options.indent_style = IndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            options.indent_style = IndentStyle::Space(size.unwrap_or(default_size));
        }
        None => {}
    }

    let is_check = session.args.contains("--check");
    let is_silent = session.args.contains("--silent");

    // Check that at least one input file / directory was specified in the command line
    let mut inputs = vec![session
        .args
        .free_from_os_str(into_os_string)
        .expect("needs at least one input file or directory")];

    while let Some(input) = session
        .args
        .opt_free_from_os_str(into_os_string)
        .expect("failed to parse argument")
    {
        inputs.push(input);
    }

    // At this point any remaining command line argument is unknown
    for arg in session.args.finish() {
        panic!("unexpected argument {arg:?}");
    }

    let (tx_files, rx_files) = channel();
    let (tx_diags, rx_diags) = channel();

    let formatted = AtomicUsize::new(0);
    let file_ids = AtomicUsize::new(0);

    let ctx = TraversalContext {
        app: &session.app,
        options,
        is_check,
        file_ids: &file_ids,
        formatted: &formatted,
        files: tx_files,
        diagnostics: tx_diags,
    };

    let start = Instant::now();

    scope(move |scope| {
        for input in inputs {
            let path = PathBuf::from(input);
            let file_id = ctx.acquire_file_id(path.clone());

            let file_type = match path.metadata().with_file_id(file_id) {
                Ok(meta) => meta.file_type(),
                Err(err) => {
                    ctx.push_diagnostic(err);
                    return;
                }
            };

            if file_type.is_file() {
                let ctx = ctx.clone();
                scope.spawn(move |_| {
                    handle_file(ctx, &path, file_id);
                });
                continue;
            }

            if file_type.is_dir() {
                let ctx = ctx.clone();
                scope.spawn(move |scope| {
                    handle_dir(scope, ctx, &path, file_id);
                });
                continue;
            }

            ctx.push_diagnostic(Diagnostic::error(file_id, "IO", "unhandled file type"));
        }
    });

    let duration = start.elapsed();
    let count = formatted.load(Ordering::Relaxed);
    println!("Formatted {count} files in {duration:?}");

    let mut file_ids = HashSet::new();
    let mut diagnostics = Vec::new();
    while let Ok(diag) = rx_diags.recv() {
        file_ids.insert(diag.file_id);
        diagnostics.push(diag);
    }

    if diagnostics.is_empty() {
        return;
    }

    if !is_silent {
        let mut files = PathFiles::default();
        while let Ok((file_id, path)) = rx_files.recv() {
            if !file_ids.contains(&file_id) {
                continue;
            }

            let name = path.display().to_string();
            let source = fs::read_to_string(path).ok().unwrap_or_else(String::new);

            files.storage.insert(file_id, SimpleFile::new(name, source));
        }

        let mut emit = Emitter::new(&files);
        for diag in diagnostics {
            emit.emit_stderr(&diag, true).unwrap();
        }
    }

    process::exit(1)
}

fn into_os_string(arg: &OsStr) -> Result<OsString, Infallible> {
    arg.try_into()
}

#[derive(Default)]
struct PathFiles {
    storage: HashMap<FileId, SimpleFile>,
}

impl Files for PathFiles {
    fn name(&self, id: FileId) -> Option<&str> {
        self.storage.get(&id)?.name(id)
    }

    fn source(&self, id: FileId) -> Option<&str> {
        self.storage.get(&id)?.source(id)
    }

    fn line_index(&self, id: FileId, byte_index: usize) -> Option<usize> {
        self.storage.get(&id)?.line_index(id, byte_index)
    }

    fn line_range(&self, file_id: FileId, line_index: usize) -> Option<Range<usize>> {
        self.storage.get(&file_id)?.line_range(file_id, line_index)
    }
}

#[derive(Clone)]
/// Context object shared between directory traversal tasks
struct TraversalContext<'a> {
    app: &'a App,
    options: FormatOptions,
    is_check: bool,
    file_ids: &'a AtomicUsize,
    formatted: &'a AtomicUsize,
    files: Sender<(FileId, PathBuf)>,
    diagnostics: Sender<Diagnostic>,
}

impl<'a> TraversalContext<'a> {
    fn acquire_file_id(&self, path: PathBuf) -> FileId {
        let file_id = self.file_ids.fetch_add(1, Ordering::Relaxed);
        self.files.send((file_id, path)).ok();
        file_id
    }

    fn add_formatted(&self) {
        self.formatted.fetch_add(1, Ordering::Relaxed);
    }

    fn push_diagnostic(&self, err: Diagnostic) {
        self.diagnostics.send(err).ok();
    }
}

/// Default list of ignored directories, in the future will be supplanted by
/// detecting and parsing .ignore files
const DEFAULT_IGNORE: &[&str] = &[".git", "node_modules"];

/// Traverse a single directory, scheduling any file for formatting and
/// directories for subsequent traversal
fn handle_dir<'a>(scope: &Scope<'a>, ctx: TraversalContext<'a>, path: &Path, file_id: FileId) {
    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
        if DEFAULT_IGNORE.contains(&file_name) {
            return;
        }
    }

    let iter = match read_dir(path).with_file_id(file_id) {
        Ok(iter) => iter,
        Err(err) => {
            ctx.push_diagnostic(err);
            return;
        }
    };

    for entry in iter {
        let entry = match entry.with_file_id(file_id) {
            Ok(entry) => entry,
            Err(err) => {
                ctx.push_diagnostic(err);
                continue;
            }
        };

        let path = entry.path();
        let file_id = ctx.acquire_file_id(path.clone());

        let file_type = match entry.file_type().with_file_id(file_id) {
            Ok(file_type) => file_type,
            Err(err) => {
                ctx.push_diagnostic(err);
                continue;
            }
        };

        if file_type.is_dir() {
            let ctx = ctx.clone();
            scope.spawn(move |scope| {
                handle_dir(scope, ctx, &path, file_id);
            });
            continue;
        }

        if file_type.is_file() {
            // Performing this check here lets us skip scheduling unsupported
            // files entirely as well as silently ignore unsupported files when
            // doing a directory traversal but printing an error message if the
            // user explicitly requests an unsupported file to be formatted
            if !ctx.app.can_format(&RomePath::new(&path)) {
                continue;
            }

            let ctx = ctx.clone();
            scope.spawn(move |_| {
                handle_file(ctx, &path, file_id);
            });
            continue;
        }

        ctx.push_diagnostic(Diagnostic::error(file_id, "IO", "unhandled file type"));
    }
}

/// This function wraps the [format_file] function implementing the formatting
/// in a [catch_unwind] block and emit diagnostics in case of error (either the
/// formatting function returns Err or panics)
fn handle_file(ctx: TraversalContext, path: &Path, file_id: FileId) {
    let params = FormatFileParams {
        app: ctx.app,
        options: ctx.options,
        is_check: ctx.is_check,
        path,
        file_id,
    };

    match catch_unwind(move || format_file(params)) {
        Ok(Ok(errors)) => {
            if errors.is_empty() {
                ctx.add_formatted();
            } else {
                for error in errors {
                    ctx.push_diagnostic(error);
                }
            }
        }
        Ok(Err(err)) => {
            ctx.push_diagnostic(err);
        }
        Err(err) => {
            let msg = match err.downcast::<String>() {
                Ok(msg) => format!("formatting panicked: {msg}"),
                Err(_) => String::from("formatting panicked"),
            };

            ctx.push_diagnostic(Diagnostic::error(file_id, "Panic", msg));
        }
    }
}

struct FormatFileParams<'a> {
    app: &'a App,
    options: FormatOptions,
    is_check: bool,
    path: &'a Path,
    file_id: FileId,
}

/// This function performs the actual formatting: it reads the file from disk
/// (or mmap it), parse and format it then either write the result back or
/// compare it to the original content and emit a diagnostic in check mode
fn format_file(params: FormatFileParams) -> Result<Vec<Diagnostic>, Diagnostic> {
    if !params.app.can_format(&RomePath::new(params.path)) {
        return Err(Diagnostic::error(
            params.file_id,
            "IO",
            "unhandled file type",
        ));
    }

    let source_type = SourceType::try_from(params.path).unwrap_or_else(|_| SourceType::js_module());

    let mut file = fs::File::options()
        .read(true)
        .write(true)
        .open(params.path)
        .with_file_id(params.file_id)?;

    let input = FileBuffer::read(&mut file, !params.is_check).with_file_id(params.file_id)?;

    let root = parse(
        input.to_str().with_file_id_and_code(params.file_id, "IO")?,
        params.file_id,
        source_type,
    );

    if root.has_errors() {
        return Ok(root.into_errors());
    }

    let result = rome_formatter::format(params.options, &root.syntax())
        .with_file_id_and_code(params.file_id, "Format")?;

    let output = result.as_code().as_bytes();

    if params.is_check {
        let has_diff = output != input.as_bytes();
        if has_diff {
            return Err(Diagnostic::error(
                params.file_id,
                "Diff",
                "File content differs from formatting output",
            ));
        }
    } else {
        // Release the mmap / buffer
        drop(input);

        // Truncate the file
        file.set_len(0).with_file_id(params.file_id)?;

        // Reset the write cursor
        file.seek(SeekFrom::Start(0)).with_file_id(params.file_id)?;

        // Write the new content
        file.write_all(output).with_file_id(params.file_id)?;
    }

    Ok(Vec::new())
}

/// Content of a file loaded into memory, internal representation can be either
/// a memory-map or a string buffer
enum FileBuffer {
    Mmap(memmap2::Mmap),
    String(String),
}

impl FileBuffer {
    fn read(file: &mut fs::File, allow_mmap: bool) -> io::Result<Self> {
        // TODO: figure out on which platforms this is useful
        if allow_mmap {
            if let Ok(mmap) = unsafe { memmap2::Mmap::map(&*file) } {
                return Ok(Self::Mmap(mmap));
            }
        }

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(Self::String(buffer))
    }

    fn as_bytes(&self) -> &[u8] {
        match self {
            FileBuffer::Mmap(mmap) => mmap.as_ref(),
            FileBuffer::String(buffer) => buffer.as_ref(),
        }
    }

    fn to_str(&self) -> Result<&str, Utf8Error> {
        match self {
            FileBuffer::Mmap(mmap) => Ok(std::str::from_utf8(mmap)?),
            FileBuffer::String(buffer) => Ok(buffer),
        }
    }
}

/// Extension trait for turning [Display]-able error types into [Diagnostic]
trait ResultExt {
    type Result;
    fn with_file_id_and_code(
        self,
        file_id: FileId,
        code: &'static str,
    ) -> Result<Self::Result, Diagnostic>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: Display,
{
    type Result = T;

    fn with_file_id_and_code(
        self,
        file_id: FileId,
        code: &'static str,
    ) -> Result<Self::Result, Diagnostic> {
        self.map_err(move |err| Diagnostic::error(file_id, code, err.to_string()))
    }
}

/// Extension trait for turning [io::Error] into [Diagnostic]
trait ResultIoExt: ResultExt {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, Diagnostic>;
}

impl<T> ResultIoExt for io::Result<T> {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, Diagnostic> {
        self.with_file_id_and_code(file_id, "IO")
    }
}
