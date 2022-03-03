use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    io::{self, Read, Seek, SeekFrom, Write},
    panic::catch_unwind,
    path::{Path, PathBuf},
    str::Utf8Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    time::Instant,
};

use rayon::{self, scope, Scope};
use rome_core::App;
use rome_formatter::{FormatError, FormatOptions, IndentStyle};
use rome_path::RomePath;
use rslint_parser::{parse, SourceType};

use crate::CliSession;

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

    let inputs = session.args.finish();
    if inputs.is_empty() {
        panic!("needs at least one input file or directory");
    }

    let ctx = TraversalContext {
        app: session.app,
        options,
        is_check,
        formatted: AtomicUsize::new(0),
        diagnostics: Mutex::default(),
    };

    let start = Instant::now();

    {
        let ctx = &ctx;
        scope(move |scope| {
            for input in inputs {
                scope.spawn(move |scope| {
                    handle_path(scope, ctx, input.into());
                });
            }
        });
    }

    let duration = start.elapsed();
    let count = ctx.formatted.load(Ordering::Relaxed);
    println!("Formatted {count} files in {duration:?}");

    for diag in ctx.into_diagnostics() {
        eprintln!("{:?}", diag);
    }
}

struct TraversalContext {
    app: App,
    options: FormatOptions,
    is_check: bool,
    formatted: AtomicUsize,
    diagnostics: Mutex<Vec<Diagnostic>>,
}

impl TraversalContext {
    fn add_formatted(&self) {
        self.formatted.fetch_add(1, Ordering::Relaxed);
    }

    fn push_diagnostic(&self, err: impl Into<Diagnostic>) {
        self.diagnostics.lock().unwrap().push(err.into());
    }

    fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics.into_inner().unwrap()
    }
}

#[derive(Debug)]
enum Diagnostic {
    Io(io::Error),
    Utf8(Utf8Error),
    Format(FormatError),
    String(String),
}

impl From<io::Error> for Diagnostic {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<Utf8Error> for Diagnostic {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<FormatError> for Diagnostic {
    fn from(err: FormatError) -> Self {
        Self::Format(err)
    }
}

const DEFAULT_IGNORE: &[&str] = &[".git", "node_modules"];

fn handle_path<'a>(scope: &Scope<'a>, ctx: &'a TraversalContext, path: PathBuf) {
    if path.is_dir() {
        if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
            if DEFAULT_IGNORE.contains(&file_name) {
                return;
            }
        }

        let iter = match read_dir(path) {
            Ok(iter) => iter,
            Err(err) => {
                ctx.push_diagnostic(err);
                return;
            }
        };

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    ctx.push_diagnostic(err);
                    continue;
                }
            };

            let path = entry.path();
            scope.spawn(move |scope| {
                handle_path(scope, ctx, path);
            });
        }

        return;
    }

    if path.is_file() {
        let result = catch_unwind(|| format_file(ctx, &path));
        match result {
            Ok(Ok(true)) => ctx.add_formatted(),
            Ok(Ok(false)) => {}
            Ok(Err(err)) => ctx.push_diagnostic(err),
            Err(err) => {
                let msg = match err.downcast::<String>() {
                    Ok(msg) => format!("formatting {path:?} panicked: {msg}"),
                    Err(_) => format!("formatting {path:?} panicked"),
                };

                ctx.push_diagnostic(Diagnostic::String(msg));
            }
        }
        return;
    }

    ctx.push_diagnostic(Diagnostic::String(String::from("unhandled file type")));
}

fn format_file(ctx: &TraversalContext, path: &Path) -> Result<bool, Diagnostic> {
    if !ctx.app.can_format(&RomePath::new(path)) {
        return Ok(false);
    }

    let source_type = SourceType::from_path(path).unwrap_or_else(SourceType::js_module);

    let mut file = fs::File::options().read(true).write(true).open(path)?;

    let input = FileBuffer::read(&mut file)?;

    let root = parse(input.to_str()?, 0, source_type).syntax();
    let result = rome_formatter::format(ctx.options, &root)?;
    let output = result.as_code().as_bytes();

    if ctx.is_check {
        let has_diff = output != input.as_bytes();
        if has_diff {
            return Err(Diagnostic::String(format!("DIFF: {path:?}")));
        }
    } else {
        // Release the mmap / buffer
        drop(input);

        // Truncate the file
        file.set_len(0)?;

        // Reset the write cursor
        file.seek(SeekFrom::Start(0))?;

        // Write the new content
        file.write_all(output)?;
    }

    Ok(true)
}

enum FileBuffer {
    Mmap(memmap2::Mmap),
    String(String),
}

impl FileBuffer {
    fn read(file: &mut fs::File) -> Result<Self, Diagnostic> {
        // TODO: figure out on which platforms this is useful
        if let Ok(mmap) = unsafe { memmap2::Mmap::map(&*file) } {
            return Ok(Self::Mmap(mmap));
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

    fn to_str(&self) -> Result<&str, Diagnostic> {
        match self {
            FileBuffer::Mmap(mmap) => Ok(std::str::from_utf8(mmap)?),
            FileBuffer::String(buffer) => Ok(buffer),
        }
    }
}
