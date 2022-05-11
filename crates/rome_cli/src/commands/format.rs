use std::{
    collections::HashMap,
    ffi::OsString,
    fmt::Display,
    io,
    panic::catch_unwind,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::join;
use rome_console::{
    codespan::{Locus, SourceFile},
    diff::{Diff, DiffMode},
    markup, Console, ConsoleExt,
};
use rome_core::Features;
use rome_diagnostics::{
    file::{FileId, Files, SimpleFile},
    Diagnostic, DiagnosticHeader, Severity,
};
use rome_fs::{AtomicInterner, FileSystem, PathInterner, RomePath};
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

    let indent_style = session
        .args
        .opt_value_from_str("--indent-style")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-style",
            source,
        })?;

    match indent_style {
        Some(IndentStyle::Tab) => {
            options.indent_style = IndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            options.indent_style = IndentStyle::Space(size.unwrap_or(default_size));
        }
        None => {}
    }

    let quote_style = session
        .args
        .opt_value_from_str("--quote-style")
        .map_err(|source| Termination::ParseError {
            argument: "--quote-style",
            source,
        })?;

    if let Some(quote_style) = quote_style {
        options.quote_style = quote_style;
    }

    let line_width = session
        .args
        .opt_value_from_str("--line-width")
        .map_err(|source| Termination::ParseError {
            argument: "--line-width",
            source,
        })?;

    if let Some(line_width) = line_width {
        options.line_width = line_width;
    }

    let is_write = session.args.contains("--write");
    let is_ci = session.args.contains("--ci");
    let ignore_errors = session.args.contains("--skip-errors");

    let mode = match (is_write, is_ci) {
        (true, true) => return Err(Termination::IncompatibleArguments("--write", "--ci")),
        (true, false) => FormatMode::Write,
        (false, true) => FormatMode::Check,
        (false, false) => FormatMode::Print,
    };

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
    let (send_msgs, recv_msgs) = unbounded();

    let formatted = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    let fs = &*session.app.fs;
    let features = &session.app.features;
    let console = &mut *session.app.console;

    let (has_errors, duration) = join(
        || print_messages_to_console(mode, console, recv_files, recv_msgs),
        || {
            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once the traversal finishes
            traverse_inputs(
                fs,
                inputs,
                &FormatCommandOptions {
                    fs,
                    features,
                    options,
                    mode,
                    ignore_errors,
                    interner,
                    formatted: &formatted,
                    skipped: &skipped,
                    messages: send_msgs,
                },
            )
        },
    );

    let count = formatted.load(Ordering::Relaxed);
    let skipped = skipped.load(Ordering::Relaxed);

    match mode {
        FormatMode::Write => {
            console.log(rome_console::markup! {
                <Info>"Formatted "{count}" files in "{duration}</Info>
            });
        }
        FormatMode::Check => {
            console.log(rome_console::markup! {
                <Info>"Checked "{count}" files in "{duration}</Info>
            });
        }
        FormatMode::Print => {
            console.log(rome_console::markup! {
                <Info>"Compared "{count}" files in "{duration}</Info>
            });
        }
    }

    if skipped > 0 {
        console.log(rome_console::markup! {
            <Warn>"Skipped "{skipped}" files"</Warn>
        });
    }

    // Formatting emitted error diagnostics, exit with a non-zero code
    if !has_errors {
        Ok(())
    } else {
        Err(Termination::FormattingError)
    }
}

/// Initiate the filesystem traversal tasks with the provided input paths and
/// run it to completion, returning the duration of the process
fn traverse_inputs(
    fs: &dyn FileSystem,
    inputs: Vec<OsString>,
    ctx: &FormatCommandOptions,
) -> Duration {
    let start = Instant::now();

    fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
        for input in inputs {
            scope.spawn(ctx, PathBuf::from(input));
        }
    }));

    start.elapsed()
}

/// This thread receives [Message]s from the workers through the `recv_msgs`
/// and `recv_files` channels and prints them to the console
fn print_messages_to_console(
    mode: FormatMode,
    console: &mut dyn Console,
    recv_files: Receiver<(usize, PathBuf)>,
    recv_msgs: Receiver<Message>,
) -> bool {
    let mut has_errors = false;
    let mut paths = HashMap::new();

    while let Ok(msg) = recv_msgs.recv() {
        match msg {
            Message::Error(err) => {
                // Retrieves the file name from the file ID cache, if it's a miss
                // flush entries from the interner channel until it's found
                let file_name = match paths.get(&err.file_id) {
                    Some(path) => Some(path),
                    None => loop {
                        match recv_files.recv() {
                            Ok((file_id, path)) => {
                                paths.insert(file_id, path.display().to_string());
                                if file_id == err.file_id {
                                    break Some(&paths[&file_id]);
                                }
                            }
                            // In case the channel disconnected without sending
                            // the path we need, print the error without a file
                            // name (normally this should never happen)
                            Err(_) => break None,
                        }
                    },
                };

                console.error(markup! {
                    {DiagnosticHeader {
                        locus: file_name.map(|name| Locus::File { name }),
                        severity: err.severity,
                        code: Some(err.code),
                        title: markup!{ {err.message} },
                    }}
                });
            }

            Message::Diagnostics {
                name,
                content,
                diagnostics,
            } => {
                let file = SimpleFile::new(name, content);

                for diag in diagnostics {
                    has_errors |= diag.is_error();
                    console.error(markup! {
                        {diag.display(&file)}
                    });
                }
            }

            Message::Diff {
                file_name,
                old,
                new,
            } => {
                let header = if matches!(mode, FormatMode::Check) {
                    // A diff is an error in CI mode
                    has_errors = true;
                    DiagnosticHeader {
                        locus: Some(Locus::File { name: &file_name }),
                        severity: Severity::Error,
                        code: Some("CI"),
                        title: markup! { "File content differs from formatting output" },
                    }
                } else {
                    DiagnosticHeader {
                        locus: Some(Locus::File { name: &file_name }),
                        severity: Severity::Help,
                        code: Some("Formatter"),
                        title: markup! { "Formatter would have printed the following content:" },
                    }
                };

                // Skip printing the diff for files over 1Mb (probably a minified file)
                let max_len = old.len().max(new.len());
                if max_len >= 1_000_000 {
                    console.error(markup! {
                        {header}"\n"
                        <Info>"[Diff not printed for file over 1Mb]\n"</Info>
                    });
                    continue;
                }

                let diff = Diff {
                    mode: DiffMode::Unified,
                    left: &old,
                    right: &new,
                };

                console.error(markup! {
                    {header}"\n"
                    {diff}
                });
            }
        }
    }

    has_errors
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
    fn source(&self, id: FileId) -> Option<SourceFile<'_>> {
        self.storage.get(&id)?.source(id)
    }
}

/// Determines how the result of formatting should be handled
#[derive(Clone, Copy)]
enum FormatMode {
    /// Write the result to the original file
    ///
    /// This is the behavior of the CLI when the `--write` argument is specified
    Write,
    /// Compare the result to the original content of the file and return an
    /// error if they differ
    ///
    /// This is the behavior of the CLI when the `--ci` argument is specified
    Check,
    /// Print a diff of the formatting result with original content of the file
    ///
    /// This is the default behavior when no CLI argument is specified
    Print,
}

/// Context object shared between directory traversal tasks
struct FormatCommandOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    fs: &'app dyn FileSystem,
    /// Set of features supported by this instance of the CLI
    features: &'ctx Features,
    /// Options to use for formatting the discovered files
    options: FormatOptions,
    /// Determines how the result of formatting should be handled
    mode: FormatMode,
    /// Whether the formatter should silently skip files with errors
    ignore_errors: bool,
    /// File paths interner used by the filesystem traversal
    interner: AtomicInterner,
    /// Shared atomic counter storing the number of formatted files
    formatted: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    messages: Sender<Message>,
}

impl<'ctx, 'app> FormatCommandOptions<'ctx, 'app> {
    /// Increment the formatted files counter
    fn add_formatted(&self) {
        self.formatted.fetch_add(1, Ordering::Relaxed);
    }

    /// Send a message to the display thread
    fn push_message(&self, msg: impl Into<Message>) {
        self.messages.send(msg.into()).ok();
    }
}

impl<'ctx, 'app> TraversalContext for FormatCommandOptions<'ctx, 'app> {
    fn interner(&self) -> &dyn PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, file_id: FileId, code: &'static str, message: String) {
        self.push_message(FormatError {
            severity: Severity::Error,
            file_id,
            code,
            message,
        });
    }

    fn can_handle(&self, rome_path: &RomePath) -> bool {
        self.features.can_format(rome_path)
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
        fs: ctx.fs,
        features: ctx.features,
        options: ctx.options,
        mode: ctx.mode,
        ignore_errors: ctx.ignore_errors,
        path,
        file_id,
    };

    match catch_unwind(move || format_file(params)) {
        Ok(Ok(None)) => {
            ctx.add_formatted();
        }
        Ok(Ok(Some(msg))) => {
            ctx.add_formatted();
            ctx.push_message(msg);
        }
        Ok(Err(err)) => {
            ctx.skipped.fetch_add(1, Ordering::Relaxed);
            ctx.push_message(err);
        }
        Err(err) => {
            let message = match err.downcast::<String>() {
                Ok(msg) => format!("formatting panicked: {msg}"),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(msg) => format!("formatting panicked: {msg}"),
                    Err(_) => String::from("formatting panicked"),
                },
            };

            ctx.push_message(FormatError {
                severity: Severity::Bug,
                file_id,
                code: "Panic",
                message,
            });
        }
    }
}

struct FormatFileParams<'ctx, 'app> {
    fs: &'app dyn FileSystem,
    features: &'ctx Features,
    options: FormatOptions,
    mode: FormatMode,
    ignore_errors: bool,
    path: &'ctx Path,
    file_id: FileId,
}

/// The return type for [format_file], with the following semantics:
/// - `Ok(None)` means the operation was successful (the file is added to the
///   `formatted` counter)
/// - `Ok(Some(_))` means the operation was successful but a message still
///   needs to be printed (eg. the diff when not in CI or write mode)
/// - `Err(_)` means the operation failed and the file should be added to the
///   skipped counter
type FormatResult = Result<Option<Message>, Message>;

/// This function performs the actual formatting: it reads the file from disk
/// (or map it into memory it), parse and format it; then, it either writes the
/// result back or compares it with the original content and emit a diagnostic
fn format_file(params: FormatFileParams) -> FormatResult {
    tracing::trace_span!("format_file", path = ?params.path).in_scope(move || {
        if !params.features.can_format(&RomePath::new(params.path)) {
            return Err(Message::from(FormatError {
                severity: Severity::Error,
                file_id: params.file_id,
                code: "IO",
                message: String::from("unhandled file type"),
            }));
        }

        let source_type =
            SourceType::try_from(params.path).unwrap_or_else(|_| SourceType::js_module());

        let mut file = params.fs.open(params.path).with_file_id(params.file_id)?;

        let mut input = String::new();
        file.read_to_string(&mut input)
            .with_file_id(params.file_id)?;

        let root = parse(&input, params.file_id, source_type);

        if root.has_errors() {
            return Err(if params.ignore_errors {
                Message::from(FormatError {
                    severity: Severity::Warning,
                    file_id: params.file_id,
                    code: "IO",
                    message: String::from("Skipped file with syntax errors"),
                })
            } else {
                Message::Diagnostics {
                    name: params.path.display().to_string(),
                    content: input,
                    diagnostics: root.into_diagnostics(),
                }
            });
        }

        let result = rome_js_formatter::format_node(params.options, &root.syntax())
            .with_file_id_and_code(params.file_id, "Format")?
            .print();

        let output = result.as_code().as_bytes();

        match params.mode {
            FormatMode::Write => {
                file.set_content(output).with_file_id(params.file_id)?;
            }
            FormatMode::Check | FormatMode::Print => {
                let has_diff = output != input.as_bytes();
                if has_diff {
                    return Ok(Some(Message::Diff {
                        file_name: params.path.display().to_string(),
                        old: input,
                        new: result.into_code(),
                    }));
                }
            }
        }

        Ok(None)
    })
}

/// Wrapper type for messages that can be printed during the formatting process
enum Message {
    Error(FormatError),
    Diagnostics {
        name: String,
        content: String,
        diagnostics: Vec<Diagnostic>,
    },
    Diff {
        file_name: String,
        old: String,
        new: String,
    },
}

impl From<FormatError> for Message {
    fn from(err: FormatError) -> Self {
        Self::Error(err)
    }
}

/// Generic error type returned by the formatting process
struct FormatError {
    severity: Severity,
    file_id: FileId,
    code: &'static str,
    message: String,
}

/// Extension trait for turning [Display]-able error types into [FormatError]
trait ResultExt {
    type Result;
    fn with_file_id_and_code(
        self,
        file_id: FileId,
        code: &'static str,
    ) -> Result<Self::Result, FormatError>;
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
    ) -> Result<Self::Result, FormatError> {
        self.map_err(move |err| FormatError {
            severity: Severity::Error,
            file_id,
            code,
            message: err.to_string(),
        })
    }
}

/// Extension trait for turning [io::Error] into [FormatError]
trait ResultIoExt: ResultExt {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, FormatError>;
}

impl<T> ResultIoExt for io::Result<T> {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, FormatError> {
        self.with_file_id_and_code(file_id, "IO")
    }
}
