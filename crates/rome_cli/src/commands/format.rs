use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io,
    ops::Range,
    panic::catch_unwind,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use crossbeam::channel::{unbounded, Sender};
use rome_console::{
    diff::{Diff, DiffMode},
    markup,
};
use rome_core::App;
use rome_diagnostics::{
    file::{FileId, Files, SimpleFile},
    Diagnostic,
};
use rome_fs::{AtomicInterner, PathInterner, RomePath};
use rome_fs::{TraversalContext, TraversalScope};
use rome_js_formatter::{FormatOptions, IndentStyle};
use rome_js_parser::{parse, SourceType};

use crate::{CliSession, Termination};

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), Termination> {
    let mut options = FormatOptions::default();

    let size = session
        .args
        .opt_value_from_str("--indent-size")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-size",
            source,
        })?;

    let style = session
        .args
        .opt_value_from_str("--indent-style")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-style",
            source,
        })?;

    match style {
        Some(IndentStyle::Tab) => {
            options.indent_style = IndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            options.indent_style = IndentStyle::Space(size.unwrap_or(default_size));
        }
        None => {}
    }

    let is_check = session.args.contains("--ci");
    let ignore_errors = session.args.contains("--skip-errors");

    // Check that at least one input file / directory was specified in the command line
    let mut inputs = vec![];

    for input in session.args.finish() {
        if let Some(maybe_arg) = input.to_str() {
            let without_dashes = maybe_arg.trim_start_matches('-');
            if without_dashes.is_empty() {
                // `-` or `--`
                continue;
            }
            // `--<some character>` or `-<some character>`
            if without_dashes != input {
                return Err(Termination::UnexpectedArgument { argument: input });
            }
        }
        inputs.push(input);
    }

    if inputs.is_empty() {
        return Err(Termination::MissingArgument {
            argument: "<INPUT>",
        });
    }

    let (interner, recv_files) = AtomicInterner::new();
    let (send_diags, recv_diags) = unbounded();

    let formatted = AtomicUsize::new(0);

    let start = Instant::now();

    {
        // The traversal context is scoped to ensure all the channels it
        // contains are properly closed once the traversal finishes
        let ctx = FormatCommandOptions {
            app: &session.app,
            options,
            is_check,
            ignore_errors,
            interner,
            formatted: &formatted,
            diagnostics: send_diags,
        };

        let ctx = &ctx;
        session
            .app
            .fs
            .traversal(Box::new(move |scope: &dyn TraversalScope| {
                for input in inputs {
                    scope.spawn(ctx, PathBuf::from(input));
                }
            }));
    }

    let duration = start.elapsed();
    let count = formatted.load(Ordering::Relaxed);
    if is_check {
        session.app.console.message(rome_console::markup! {
            <Info>"Checked "{count}" files in "{duration}</Info>
        });
    } else {
        session.app.console.message(rome_console::markup! {
            <Info>"Formatted "{count}" files in "{duration}</Info>
        });
    }

    let mut has_errors = false;
    let mut file_ids = HashSet::new();
    let mut diagnostics = Vec::new();

    while let Ok(error) = recv_diags.recv() {
        match &error {
            Error::Diagnostic(diag) => {
                has_errors |= diag.is_error();
                file_ids.insert(diag.file_id);
            }
            Error::Diff { .. } => {
                has_errors = true;
            }
        }

        diagnostics.push(error);
    }

    let mut files = PathFiles::default();
    while let Ok((file_id, path)) = recv_files.recv() {
        if !file_ids.contains(&file_id) {
            continue;
        }

        let name = path.display().to_string();
        let mut source = String::new();
        session
            .app
            .fs
            .open(&path)
            .and_then(|mut file| file.read_to_string(&mut source))
            // Any potential read error is ignored for two reasons:
            // - The first is that if this code is reached this means an error
            // diagnostic was emitted for this path, we can't really know what
            // it was at this stage since its an opaque diagnostic but it could
            // be that the file doesn't exist, is a directory, or the process
            // doesn't have the permission to read it. There's a fairly high
            // chance the same error will happen again when the file is loaded
            // a second time, in which case we don't want to do anything with
            // it since we're already in the process of printing the error
            // diagnostic to the console anyway and we don't want to show the
            // same error twice
            // - The second scenario is that the filesystem could be in an
            // inconsistent state, for instance the file got deleted between
            // the moment the diagnostic was emitted and the moment it gets
            // printed. The probability of this happening is very low so for
            // now the diagnostics just gets printed in "degraded mode" with no
            // code span information, and not print any additional error.
            // Eventually this will go away when the virtual filesystem can
            // cache the content of files in memory and we don't have to load
            // the file a second time to print diagnostics
            .ok();

        files.storage.insert(file_id, SimpleFile::new(name, source));
    }

    for error in diagnostics {
        match error {
            Error::Diagnostic(diag) => {
                session.app.console.diagnostic(&files, &diag);
            }
            Error::Diff {
                file_name,
                old,
                new,
            } => {
                // Skip printing the diff for files over 1Mb (probably a minified file)
                let max_len = old.len().max(new.len());
                if max_len >= 1_000_000 {
                    session.app.console.message(markup! {
                        {file_name}": "
                        <Error>"error[CI]"</Error>": File content differs from formatting output\n"
                        <Info>"[Diff not printed for file over 1Mb]\n"</Info>
                    });
                    continue;
                }

                let diff = Diff {
                    mode: DiffMode::Unified,
                    left: &old,
                    right: &new,
                };

                session.app.console.message(markup! {
                    {file_name}": "
                    <Error>"error[CI]"</Error>": File content differs from formatting output\n"
                    {diff}
                });
            }
        }
    }

    // Formatting emitted error diagnostics, exit with a non-zero code
    if !has_errors {
        Ok(())
    } else {
        Err(Termination::FormattingError)
    }
}

/// Implementation of [Files] with pre-allocated file IDs
#[derive(Default)]
struct PathFiles {
    storage: HashMap<FileId, SimpleFile>,
}

impl Files for PathFiles {
    /// Returns the name of the file identified by the id.
    fn name(&self, id: FileId) -> Option<&str> {
        self.storage.get(&id)?.name(id)
    }

    /// Returns the source of the file identified by the id.
    fn source(&self, id: FileId) -> Option<&str> {
        self.storage.get(&id)?.source(id)
    }

    /// The index of the line at the byte index.
    fn line_index(&self, id: FileId, byte_index: usize) -> Option<usize> {
        self.storage.get(&id)?.line_index(id, byte_index)
    }

    /// The byte range of line in the source of the file.
    fn line_range(&self, file_id: FileId, line_index: usize) -> Option<Range<usize>> {
        self.storage.get(&file_id)?.line_range(file_id, line_index)
    }
}

/// Context object shared between directory traversal tasks
struct FormatCommandOptions<'ctx, 'app> {
    /// Shared instance of [App]
    app: &'ctx App<'app>,
    /// Options to use for formatting the discovered files
    options: FormatOptions,
    /// Boolean flag storing whether the command is being run in check mode
    is_check: bool,
    /// Whether the formatter should silently skip files with errors
    ignore_errors: bool,
    /// File paths interner used by the filesystem traversal
    interner: AtomicInterner,
    /// Shared atomic counter storing the number of formatted files
    formatted: &'ctx AtomicUsize,
    /// Channel sending diagnostics to the display thread
    diagnostics: Sender<Error>,
}

impl<'ctx, 'app> FormatCommandOptions<'ctx, 'app> {
    /// Increment the formatted files counter
    fn add_formatted(&self) {
        self.formatted.fetch_add(1, Ordering::Relaxed);
    }

    /// Send a diagnostic to the display thread
    fn push_diagnostic(&self, err: Error) {
        self.diagnostics.send(err).ok();
    }
}

impl<'ctx, 'app> TraversalContext for FormatCommandOptions<'ctx, 'app> {
    fn interner(&self) -> &dyn PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, file_id: FileId, code: &'static str, title: String) {
        self.push_diagnostic(Diagnostic::error(file_id, code, title).into());
    }

    fn can_handle(&self, rome_path: &RomePath) -> bool {
        self.app.can_format(rome_path)
    }

    fn handle_file(&self, path: &Path, file_id: FileId) {
        handle_file(self, path, file_id)
    }
}

/// This function wraps the [format_file] function implementing the formatting
/// in a [catch_unwind] block and emit diagnostics in case of error (either the
/// formatting function returns Err or panics)
fn handle_file(ctx: &FormatCommandOptions, path: &Path, file_id: FileId) {
    let params = FormatFileParams {
        app: ctx.app,
        options: ctx.options,
        is_check: ctx.is_check,
        ignore_errors: ctx.ignore_errors,
        path,
        file_id,
    };

    match catch_unwind(move || format_file(params)) {
        Ok(Ok(errors)) => {
            if errors.is_empty() {
                ctx.add_formatted();
            } else {
                for error in errors {
                    ctx.push_diagnostic(error.into());
                }
            }
        }
        Ok(Err(err)) => {
            ctx.push_diagnostic(err);
        }
        Err(err) => {
            let msg = match err.downcast::<String>() {
                Ok(msg) => format!("formatting panicked: {msg}"),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(msg) => format!("formatting panicked: {msg}"),
                    Err(_) => String::from("formatting panicked"),
                },
            };

            ctx.push_diagnostic(Diagnostic::error(file_id, "Panic", msg).into());
        }
    }
}

struct FormatFileParams<'ctx, 'app> {
    app: &'ctx App<'app>,
    options: FormatOptions,
    is_check: bool,
    ignore_errors: bool,
    path: &'ctx Path,
    file_id: FileId,
}

/// This function performs the actual formatting: it reads the file from disk
/// (or map it into memory it), parse and format it; then, it either writes the
/// result back or compares it with the original content and emit a diagnostic
#[tracing::instrument(level = "trace", skip_all, fields(path = ?params.path))]
fn format_file(params: FormatFileParams) -> Result<Vec<Diagnostic>, Error> {
    if !params.app.can_format(&RomePath::new(params.path)) {
        return Err(Error::from(Diagnostic::error(
            params.file_id,
            "IO",
            "unhandled file type",
        )));
    }

    let source_type = SourceType::try_from(params.path).unwrap_or_else(|_| SourceType::js_module());

    let mut file = params
        .app
        .fs
        .open(params.path)
        .with_file_id(params.file_id)?;

    let mut input = String::new();
    file.read_to_string(&mut input)
        .with_file_id(params.file_id)?;

    let root = parse(&input, params.file_id, source_type);

    if root.has_errors() {
        return Ok(if params.ignore_errors {
            vec![Diagnostic::warning(
                params.file_id,
                "IO",
                "Skipped file with syntax errors",
            )]
        } else {
            root.into_diagnostics()
        });
    }

    let result = rome_js_formatter::format(params.options, &root.syntax())
        .with_file_id_and_code(params.file_id, "Format")?;

    let output = result.as_code().as_bytes();

    if params.is_check {
        let has_diff = output != input.as_bytes();
        if has_diff {
            return Err(Error::Diff {
                file_name: params.path.display().to_string(),
                old: input,
                new: result.into_code(),
            });
        }
    } else {
        file.set_content(output).with_file_id(params.file_id)?;
    }

    Ok(Vec::new())
}

/// Wrapper type for errors that may happen during the formatting process
enum Error {
    Diagnostic(Diagnostic),
    Diff {
        file_name: String,
        old: String,
        new: String,
    },
}

impl From<Diagnostic> for Error {
    fn from(diagnostic: Diagnostic) -> Self {
        Self::Diagnostic(diagnostic)
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
