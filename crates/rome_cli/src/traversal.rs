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
use rome_analyze::{analyze, AnalysisFilter, RuleCategories};
use rome_console::{
    codespan::Locus,
    diff::{Diff, DiffMode},
    markup, Console, ConsoleExt,
};
use rome_core::Features;
use rome_diagnostics::{
    file::{FileId, SimpleFile},
    Diagnostic, DiagnosticHeader, Severity,
};
use rome_fs::{AtomicInterner, FileSystem, PathInterner, RomePath};
use rome_fs::{TraversalContext, TraversalScope};
use rome_js_formatter::format_node;
use rome_js_formatter::options::JsFormatOptions;
use rome_js_parser::{parse, SourceType};
use rome_rowan::AstNode;

use crate::{CliSession, Termination};

pub(crate) fn traverse(mode: TraversalMode, mut session: CliSession) -> Result<(), Termination> {
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

    let processed = AtomicUsize::new(0);
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
                &TraversalOptions {
                    fs,
                    features,
                    mode,
                    interner,
                    processed: &processed,
                    skipped: &skipped,
                    messages: send_msgs,
                },
            )
        },
    );

    let count = processed.load(Ordering::Relaxed);
    let skipped = skipped.load(Ordering::Relaxed);

    match mode {
        TraversalMode::Check | TraversalMode::CI { .. } => {
            console.log(rome_console::markup! {
                <Info>"Checked "{count}" files in "{duration}</Info>
            });
        }
        TraversalMode::Format { write: false, .. } => {
            console.log(rome_console::markup! {
                <Info>"Compared "{count}" files in "{duration}</Info>
            });
        }
        TraversalMode::Format { write: true, .. } => {
            console.log(rome_console::markup! {
                <Info>"Formatted "{count}" files in "{duration}</Info>
            });
        }
    }

    if skipped > 0 {
        console.log(rome_console::markup! {
            <Warn>"Skipped "{skipped}" files"</Warn>
        });
    }

    // Processing emitted error diagnostics, exit with a non-zero code
    if !has_errors {
        Ok(())
    } else {
        Err(Termination::CheckError)
    }
}

/// Initiate the filesystem traversal tasks with the provided input paths and
/// run it to completion, returning the duration of the process
fn traverse_inputs(fs: &dyn FileSystem, inputs: Vec<OsString>, ctx: &TraversalOptions) -> Duration {
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
    mode: TraversalMode,
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
                let header = if matches!(mode, TraversalMode::CI { .. }) {
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

#[derive(Clone, Copy)]
pub(crate) enum TraversalMode {
    Check,
    CI {
        options: JsFormatOptions,
    },
    Format {
        options: JsFormatOptions,
        ignore_errors: bool,
        write: bool,
    },
}

/// Context object shared between directory traversal tasks
struct TraversalOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    fs: &'app dyn FileSystem,
    /// Set of features supported by this instance of the CLI
    features: &'ctx Features,
    /// Determines how the files should be processed
    mode: TraversalMode,
    /// File paths interner used by the filesystem traversal
    interner: AtomicInterner,
    /// Shared atomic counter storing the number of processed files
    processed: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    messages: Sender<Message>,
}

impl<'ctx, 'app> TraversalOptions<'ctx, 'app> {
    /// Send a message to the display thread
    fn push_message(&self, msg: impl Into<Message>) {
        self.messages.send(msg.into()).ok();
    }
}

impl<'ctx, 'app> TraversalContext for TraversalOptions<'ctx, 'app> {
    fn interner(&self) -> &dyn PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, file_id: FileId, code: &'static str, message: String) {
        self.push_message(TraversalError {
            severity: Severity::Error,
            file_id,
            code,
            message,
        });
    }

    fn can_handle(&self, rome_path: &RomePath) -> bool {
        let can_lint = self.features.can_lint(rome_path);
        let can_format = self.features.can_format(rome_path);

        match self.mode {
            TraversalMode::Check => can_lint,
            TraversalMode::CI { .. } => can_lint || can_format,
            TraversalMode::Format { .. } => can_format,
        }
    }

    fn handle_file(&self, path: &Path, file_id: FileId) {
        handle_file(self, path, file_id)
    }
}

/// This function wraps the [process_file] function implementing the traversal
/// in a [catch_unwind] block and emit diagnostics in case of error (either the
/// traversal function returns Err or panics)
fn handle_file(ctx: &TraversalOptions, path: &Path, file_id: FileId) {
    match catch_unwind(move || process_file(ctx, path, file_id)) {
        Ok(Ok(None)) => {
            ctx.processed.fetch_add(1, Ordering::Relaxed);
        }
        Ok(Ok(Some(msg))) => {
            ctx.processed.fetch_add(1, Ordering::Relaxed);
            ctx.push_message(msg);
        }
        Ok(Err(err)) => {
            ctx.skipped.fetch_add(1, Ordering::Relaxed);
            ctx.push_message(err);
        }
        Err(err) => {
            let message = match err.downcast::<String>() {
                Ok(msg) => format!("processing panicked: {msg}"),
                Err(err) => match err.downcast::<&'static str>() {
                    Ok(msg) => format!("processing panicked: {msg}"),
                    Err(_) => String::from("processing panicked"),
                },
            };

            ctx.push_message(TraversalError {
                severity: Severity::Bug,
                file_id,
                code: "Panic",
                message,
            });
        }
    }
}

/// The return type for [process_file], with the following semantics:
/// - `Ok(None)` means the operation was successful (the file is added to the
///   `processed` counter)
/// - `Ok(Some(_))` means the operation was successful but a message still
///   needs to be printed (eg. the diff when not in CI or write mode)
/// - `Err(_)` means the operation failed and the file should be added to the
///   skipped counter
type FileResult = Result<Option<Message>, Message>;

/// This function performs the actual processing: it reads the file from disk
/// and parse it; analyze and / or format it; then it either fails if error
/// diagnostics where emitted, or compare the formatted code with the original
/// content of the file and emit a diff or write the new content to the disk if
/// write mode is enabled
fn process_file(ctx: &TraversalOptions, path: &Path, file_id: FileId) -> FileResult {
    tracing::trace_span!("process_file", path = ?path).in_scope(move || {
        let rome_path = RomePath::new(path);
        let can_lint = ctx.features.can_lint(&rome_path);
        let can_format = ctx.features.can_format(&rome_path);

        let can_handle = match ctx.mode {
            TraversalMode::Check => can_lint,
            TraversalMode::CI { .. } => can_lint || can_format,
            TraversalMode::Format { .. } => can_format,
        };

        if !can_handle {
            return Err(Message::from(TraversalError {
                severity: Severity::Error,
                file_id,
                code: "IO",
                message: String::from("unhandled file type"),
            }));
        }

        let source_type = SourceType::try_from(path).unwrap_or_else(|_| SourceType::js_module());

        let mut file = ctx.fs.open(path).with_file_id(file_id)?;

        let mut input = String::new();
        file.read_to_string(&mut input).with_file_id(file_id)?;

        let parsed = parse(&input, file_id, source_type);

        // In formatting mode, abort immediately if the file has errors
        match ctx.mode {
            TraversalMode::Format { ignore_errors, .. } if parsed.has_errors() => {
                return Err(if ignore_errors {
                    Message::from(TraversalError {
                        severity: Severity::Warning,
                        file_id,
                        code: "IO",
                        message: String::from("Skipped file with syntax errors"),
                    })
                } else {
                    Message::Diagnostics {
                        name: path.display().to_string(),
                        content: input,
                        diagnostics: parsed.into_diagnostics(),
                    }
                });
            }

            _ => {}
        }

        let tree = parsed.tree();
        let mut diagnostics = parsed.into_diagnostics();

        if can_lint && matches!(ctx.mode, TraversalMode::Check | TraversalMode::CI { .. }) {
            let filter = AnalysisFilter {
                // Only run the syntax and lint rules, ignore refactors
                categories: RuleCategories::SYNTAX | RuleCategories::LINT,
                ..AnalysisFilter::default()
            };

            analyze(file_id, &tree, filter, |signal| {
                if let Some(mut diag) = signal.diagnostic() {
                    if let Some(action) = signal.action() {
                        diag.suggestions.push(action.into());
                    }

                    diagnostics.push(diag);
                }
            });
        }

        // Return early if the file has error diagnostics, there's a good
        // chance it would cause formatting errors
        let has_errors = diagnostics
            .iter()
            .any(|diag| diag.severity >= Severity::Error);

        let result = if diagnostics.is_empty() {
            None
        } else {
            Some(Message::Diagnostics {
                name: path.display().to_string(),
                content: input.clone(),
                diagnostics,
            })
        };

        if has_errors {
            // Having errors is considered a "success" at this point because
            // this is only reachable on the check / CI path (the parser result
            // is checked for errors earlier on the format path, and that mode
            // doesn't run the analyzer so no new diagnostics could have been
            // added), and having errors on these paths still means the file
            // was processed (added to the checked files counter)
            return Ok(result);
        }

        if can_format {
            let (options, write) = match ctx.mode {
                // In check mode do not run the formatter and return the result immediately
                TraversalMode::Check => return Ok(result),
                TraversalMode::CI { options } => (options, false),
                TraversalMode::Format { options, write, .. } => (options, write),
            };

            let printed = format_node(options, tree.syntax())
                .with_file_id_and_code(file_id, "Format")?
                .print();

            let output = printed.as_code().as_bytes();
            let has_diff = output != input.as_bytes();

            if has_diff {
                if write {
                    file.set_content(output).with_file_id(file_id)?;
                } else {
                    // Returning the diff message will discard the content of
                    // diagnostics, meaning those would not be printed so they
                    // have to be manually sent through the console channel
                    if let Some(msg) = result {
                        ctx.messages.send(msg).ok();
                    }

                    return Ok(Some(Message::Diff {
                        file_name: path.display().to_string(),
                        old: input,
                        new: printed.into_code(),
                    }));
                }
            }
        }

        Ok(result)
    })
}

/// Wrapper type for messages that can be printed during the traversal process
enum Message {
    Error(TraversalError),
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

impl From<TraversalError> for Message {
    fn from(err: TraversalError) -> Self {
        Self::Error(err)
    }
}

/// Generic error type returned by the traversal process
struct TraversalError {
    severity: Severity,
    file_id: FileId,
    code: &'static str,
    message: String,
}

/// Extension trait for turning [Display]-able error types into [TraversalError]
trait ResultExt {
    type Result;
    fn with_file_id_and_code(
        self,
        file_id: FileId,
        code: &'static str,
    ) -> Result<Self::Result, TraversalError>;
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
    ) -> Result<Self::Result, TraversalError> {
        self.map_err(move |err| TraversalError {
            severity: Severity::Error,
            file_id,
            code,
            message: err.to_string(),
        })
    }
}

/// Extension trait for turning [io::Error] into [TraversalError]
trait ResultIoExt: ResultExt {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, TraversalError>;
}

impl<T> ResultIoExt for io::Result<T> {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, TraversalError> {
        self.with_file_id_and_code(file_id, "IO")
    }
}
