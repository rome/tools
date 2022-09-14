use crate::{
    CliSession, Execution, FormatterReportFileDetail, FormatterReportSummary, Report,
    ReportDiagnostic, ReportDiff, ReportErrorKind, ReportKind, Termination, TraversalMode,
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rome_console::{
    codespan::Locus,
    diff::{Diff, DiffMode},
    markup, Console, ConsoleExt,
};
use rome_diagnostics::{
    file::{FileId, SimpleFile},
    Diagnostic, DiagnosticHeader, Severity, MAXIMUM_DISPLAYABLE_DIAGNOSTICS,
};
use rome_fs::{AtomicInterner, FileSystem, OpenOptions, PathInterner, RomePath};
use rome_fs::{TraversalContext, TraversalScope};
use rome_service::workspace::{SupportsFeatureResult, UnsupportedReason};
use rome_service::{
    workspace::{
        FeatureName, FileGuard, Language, OpenFileParams, RuleCategories, SupportsFeatureParams,
    },
    RomeError, Workspace,
};
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

pub(crate) fn traverse(execution: Execution, mut session: CliSession) -> Result<(), Termination> {
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

    if inputs.is_empty() && execution.as_stdin_file().is_none() {
        return Err(Termination::MissingArgument {
            argument: "<INPUT>",
        });
    }

    let (interner, recv_files) = AtomicInterner::new();
    let (send_msgs, recv_msgs) = unbounded();
    let (sender_reports_from_traversal, receiver_reports) = unbounded();

    let processed = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    let fs = &*session.app.fs;
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;

    let mut has_errors = None;
    let mut duration = None;
    let mut report = None;
    let sender_reports_from_messages = sender_reports_from_traversal.clone();
    rayon::scope(|s| {
        s.spawn(|_| {
            report = Some(collect_reports(receiver_reports));
        });

        s.spawn(|_| {
            has_errors = Some(process_messages(ProcessMessagesOptions {
                execution: execution.clone(),
                console,
                recv_files,
                recv_msgs,
                sender_reports: sender_reports_from_messages,
            }));
        });
        s.spawn(|_| {
            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once the traversal finishes
            duration = Some(traverse_inputs(
                fs,
                inputs,
                &TraversalOptions {
                    fs,
                    workspace,
                    execution: execution.clone(),
                    interner,
                    processed: &processed,
                    skipped: &skipped,
                    messages: send_msgs,
                    sender_reports: sender_reports_from_traversal,
                },
            ));
        })
    });

    let count = processed.load(Ordering::Relaxed);
    let skipped = skipped.load(Ordering::Relaxed);

    let to_terminal = execution.should_report_to_terminal();

    if let Some(duration) = duration {
        if to_terminal {
            match execution.traversal_mode() {
                TraversalMode::Check { .. } => {
                    if execution.as_fix_file_mode().is_some() {
                        console.log(markup! {
                            <Info>"Fixed "{count}" files in "{duration}</Info>
                        });
                    } else {
                        console.log(markup! {
                            <Info>"Checked "{count}" files in "{duration}</Info>
                        });
                    }
                }
                TraversalMode::CI { .. } => {
                    console.log(markup! {
                        <Info>"Checked "{count}" files in "{duration}</Info>
                    });
                }
                TraversalMode::Format { write: false, .. } => {
                    if to_terminal {
                        console.log(markup! {
                            <Info>"Compared "{count}" files in "{duration}</Info>
                        });
                    }
                }
                TraversalMode::Format { write: true, .. } => {
                    console.log(markup! {
                        <Info>"Formatted "{count}" files in "{duration}</Info>
                    });
                }
            }
        } else if let Some(mut report) = report {
            if let TraversalMode::Format { write, .. } = execution.traversal_mode() {
                let mut summary = FormatterReportSummary::default();
                if *write {
                    summary.set_files_written(count);
                } else {
                    summary.set_files_compared(count);
                }
                report.set_formatter_summary(summary);
            }

            let to_print = report.as_serialized_reports()?;
            console.log(markup! {
                {to_print}
            });
            return Ok(());
        }
    }

    if skipped > 0 {
        console.log(markup! {
            <Warn>"Skipped "{skipped}" files"</Warn>
        });
    }

    // Processing emitted error diagnostics, exit with a non-zero code
    if matches!(has_errors, Some(true)) {
        Err(Termination::CheckError)
    } else {
        Ok(())
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

struct ProcessMessagesOptions<'ctx> {
    ///  Execution of the traversal
    execution: Execution,
    /// Mutable reference to the [console](Console)
    console: &'ctx mut dyn Console,
    /// Receiver channel that expects info when a file is processed
    recv_files: Receiver<(FileId, PathBuf)>,
    /// Receiver channel that expects info when a message is sent
    recv_msgs: Receiver<Message>,
    /// Sender of reports
    sender_reports: Sender<ReportKind>,
}

/// This thread receives [Message]s from the workers through the `recv_msgs`
/// and `recv_files` channels and handles them based on [Execution]
fn process_messages(options: ProcessMessagesOptions) -> bool {
    let ProcessMessagesOptions {
        execution: mode,
        console,
        recv_files,
        recv_msgs,
        sender_reports,
    } = options;

    let mut has_errors = false;
    let mut paths = HashMap::new();
    let mut printed_diagnostics: u16 = 0;
    let mut not_printed_diagnostics = 0;
    let mut total_skipped_suggested_fixes = 0;

    while let Ok(msg) = recv_msgs.recv() {
        match msg {
            Message::SkippedFixes {
                skipped_suggested_fixes,
            } => {
                total_skipped_suggested_fixes += skipped_suggested_fixes;
            }

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

                if mode.should_report_to_terminal() {
                    console.error(markup! {
                        {DiagnosticHeader {
                            locus: file_name.map(|name| Locus::File { name }),
                            severity: err.severity,
                            code: Some(markup!({ err.code })),
                            title: markup! { {err.message} },
                        }}
                    });
                } else {
                    sender_reports
                        .send(ReportKind::Error(
                            file_name.unwrap().to_string(),
                            ReportErrorKind::Diagnostic(ReportDiagnostic {
                                code: Some(err.code.to_string()),
                                title: err.message,
                                severity: err.severity,
                            }),
                        ))
                        .ok();
                }
            }

            Message::Diagnostics {
                name,
                content,
                diagnostics,
            } => {
                let file = SimpleFile::new(name.clone(), content);
                // The command `rome check` gives a default value of 20.
                // In case of other commands that pass here, we limit to 50 to avoid to delay the terminal.
                // Once `--max-diagnostics` will be a global argument, `unwrap_of_default` should be enough.
                let max_diagnostics = mode
                    .get_max_diagnostics()
                    .unwrap_or(MAXIMUM_DISPLAYABLE_DIAGNOSTICS);
                // is CI mode we want to print all the diagnostics
                if mode.is_ci() {
                    for diag in diagnostics {
                        has_errors |= diag.is_error();
                        console.error(markup! {
                            {diag.display(&file)}
                        });
                    }
                } else {
                    for diag in diagnostics {
                        has_errors |= diag.is_error();
                        if printed_diagnostics < max_diagnostics {
                            if mode.should_report_to_terminal() {
                                console.error(markup! {
                                    {diag.display(&file)}
                                });
                            }
                            printed_diagnostics += 1;
                        } else {
                            not_printed_diagnostics += 1;
                        }

                        if !mode.should_report_to_terminal() {
                            sender_reports
                                .send(ReportKind::Error(
                                    name.to_string(),
                                    ReportErrorKind::Diagnostic(ReportDiagnostic {
                                        code: diag.code,
                                        title: String::from("test here"),
                                        severity: diag.severity,
                                    }),
                                ))
                                .ok();
                        }
                    }
                }
            }

            Message::Diff {
                file_name,
                old,
                new,
            } => {
                let header = if mode.is_ci() {
                    // A diff is an error in CI mode
                    has_errors = true;

                    DiagnosticHeader {
                        locus: Some(Locus::File { name: &file_name }),
                        severity: Severity::Error,
                        code: Some(markup!("CI")),
                        title: markup! { "File content differs from formatting output" },
                    }
                } else {
                    DiagnosticHeader {
                        locus: Some(Locus::File { name: &file_name }),
                        severity: Severity::Help,
                        code: Some(markup!("Formatter")),
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

                if mode.should_report_to_terminal() {
                    console.error(markup! {
                        {header}"\n"
                        {diff}
                    });
                } else {
                    sender_reports
                        .send(ReportKind::Error(
                            file_name.to_string(),
                            ReportErrorKind::Diff(ReportDiff {
                                before: old.to_string(),
                                after: new.to_string(),
                                severity: Severity::Error,
                            }),
                        ))
                        .ok();
                }
            }
        }
    }

    if mode.is_check() && total_skipped_suggested_fixes > 0 {
        console.log(markup! {
            <Warn>"Skipped "{total_skipped_suggested_fixes}" suggested fixes.\n"</Warn>
            <Info>"If you wish to apply the suggested fixes, use the command "<Emphasis>"rome check --apply-suggested\n"</Emphasis></Info>
        })
    }

    if !mode.is_ci() && not_printed_diagnostics > 0 {
        console.log(markup! {
            <Warn>"The number of diagnostics exceeds the number allowed by Rome.\n"</Warn>
            <Info>"Diagnostics not shown: "</Info><Emphasis>{not_printed_diagnostics}</Emphasis><Info>"."</Info>
        })
    }

    has_errors
}

fn collect_reports(receiver: Receiver<ReportKind>) -> Report {
    let mut report = Report::default();
    while let Ok(stat) = receiver.recv() {
        report.push_detail_report(stat);
    }

    report
}

/// Context object shared between directory traversal tasks
struct TraversalOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    workspace: &'ctx dyn Workspace,
    /// Determines how the files should be processed
    execution: Execution,
    /// File paths interner used by the filesystem traversal
    interner: AtomicInterner,
    /// Shared atomic counter storing the number of processed files
    processed: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    messages: Sender<Message>,
    /// Channel sending reports to the reports thread
    sender_reports: Sender<ReportKind>,
}

impl<'ctx, 'app> TraversalOptions<'ctx, 'app> {
    /// Send a message to the display thread
    fn push_message(&self, msg: impl Into<Message>) {
        self.messages.send(msg.into()).ok();
    }

    fn can_format(&self, rome_path: &RomePath) -> Result<SupportsFeatureResult, RomeError> {
        self.workspace.supports_feature(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeatureName::Format,
        })
    }

    fn push_format_stat(&self, path: String, stat: FormatterReportFileDetail) {
        self.sender_reports
            .send(ReportKind::Formatter(path, stat))
            .ok();
    }

    fn can_lint(&self, rome_path: &RomePath) -> Result<SupportsFeatureResult, RomeError> {
        self.workspace.supports_feature(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeatureName::Lint,
        })
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
        let result = match self.execution.traversal_mode() {
            TraversalMode::Check { .. } => self.can_lint(rome_path),
            TraversalMode::CI { .. } => self
                .can_lint(rome_path)
                .or_else(|_| self.can_format(rome_path)),
            TraversalMode::Format { .. } => self.can_format(rome_path),
        };

        match result {
            Ok(result) => result.reason.is_none(),
            Err(err) => {
                self.push_diagnostic(rome_path.file_id(), "IO", err.to_string());
                false
            }
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
        Ok(Ok(FileStatus::Success)) => {
            ctx.processed.fetch_add(1, Ordering::Relaxed);
        }
        Ok(Ok(FileStatus::Message(msg))) => {
            ctx.processed.fetch_add(1, Ordering::Relaxed);
            ctx.push_message(msg);
        }
        Ok(Ok(FileStatus::Ignored)) => {}
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

enum FileStatus {
    Success,
    Message(Message),
    Ignored,
}

/// The return type for [process_file], with the following semantics:
/// - `Ok(Success)` means the operation was successful (the file is added to
///   the `processed` counter)
/// - `Ok(Message(_))` means the operation was successful but a message still
///   needs to be printed (eg. the diff when not in CI or write mode)
/// - `Ok(Ignored)` means the file was ignored (the file is not added to the
///   `processed` or `skipped` counters)
/// - `Err(_)` means the operation failed and the file should be added to the
///   `skipped` counter
type FileResult = Result<FileStatus, Message>;

/// This function performs the actual processing: it reads the file from disk
/// and parse it; analyze and / or format it; then it either fails if error
/// diagnostics where emitted, or compare the formatted code with the original
/// content of the file and emit a diff or write the new content to the disk if
/// write mode is enabled
fn process_file(ctx: &TraversalOptions, path: &Path, file_id: FileId) -> FileResult {
    tracing::trace_span!("process_file", path = ?path).in_scope(move || {
        let rome_path = RomePath::new(path, file_id);
        let supported_format = ctx
            .can_format(&rome_path)
            .with_file_id_and_code(file_id, "IO")?;
        let supported_lint = ctx
            .can_lint(&rome_path)
            .with_file_id_and_code(file_id, "IO")?;
        let supported_file = match ctx.execution.traversal_mode() {
            TraversalMode::Check { .. } => supported_lint.reason.as_ref(),
            TraversalMode::CI { .. } => supported_lint
                .reason
                .as_ref()
                .and(supported_format.reason.as_ref()),
            TraversalMode::Format { .. } => supported_format.reason.as_ref(),
        };

        if let Some(reason) = supported_file {
            return match reason {
                UnsupportedReason::FileNotSupported => Err(Message::from(TraversalError {
                    severity: Severity::Error,
                    file_id,
                    code: "IO",
                    message: String::from("unhandled file type"),
                })),
                UnsupportedReason::FeatureNotEnabled | UnsupportedReason::Ignored => {
                    Ok(FileStatus::Ignored)
                }
            };
        }
        let open_options = OpenOptions::default().read(true).write(true);
        let mut file = ctx
            .fs
            .open_with_options(path, open_options)
            .with_file_id(file_id)?;

        let mut input = String::new();
        file.read_to_string(&mut input).with_file_id(file_id)?;

        let file_guard = FileGuard::open(
            ctx.workspace,
            OpenFileParams {
                path: rome_path,
                version: 0,
                content: input.clone(),
                language_hint: Language::default(),
            },
        )
        .with_file_id_and_code(file_id, "IO")?;

        if let Some(fix_mode) = ctx.execution.as_fix_file_mode() {
            let fixed = file_guard
                .fix_file(*fix_mode)
                .with_file_id_and_code(file_id, "Lint")?;

            ctx.push_message(Message::SkippedFixes {
                skipped_suggested_fixes: fixed.skipped_suggested_fixes,
            });

            if fixed.code != input {
                file.set_content(fixed.code.as_bytes())
                    .with_file_id(file_id)?;

                return Ok(FileStatus::Success);
            }

            // If the file isn't changed, do not increment the "fixed files" counter
            return Ok(FileStatus::Ignored);
        }

        let categories = if ctx.execution.is_format() || supported_lint.reason.is_some() {
            RuleCategories::SYNTAX
        } else {
            RuleCategories::SYNTAX | RuleCategories::LINT
        };

        let result = file_guard
            .pull_diagnostics(categories)
            .with_file_id_and_code(file_id, "Lint")?;

        let has_errors = result
            .diagnostics
            .iter()
            .any(|diag| diag.severity >= Severity::Error);

        // In formatting mode, abort immediately if the file has errors
        match ctx.execution.traversal_mode() {
            TraversalMode::Format { ignore_errors, .. } if has_errors => {
                return Err(if *ignore_errors {
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
                        diagnostics: result.diagnostics,
                    }
                });
            }

            _ => {}
        }

        // In format mode the diagnostics have already been checked for errors
        // at this point, so they can just be dropped now since we don't want
        // to print syntax warnings for the format command
        let result = if result.diagnostics.is_empty() || ctx.execution.is_format() {
            FileStatus::Success
        } else {
            FileStatus::Message(Message::Diagnostics {
                name: path.display().to_string(),
                content: input.clone(),
                diagnostics: result.diagnostics,
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

        if supported_format.reason.is_none() {
            let write = match ctx.execution.traversal_mode() {
                // In check mode do not run the formatter and return the result immediately,
                // but only if the argument `--apply` is not passed.
                TraversalMode::Check { .. } => {
                    if ctx.execution.as_fix_file_mode().is_some() {
                        true
                    } else {
                        return Ok(result);
                    }
                }
                TraversalMode::CI { .. } => false,
                TraversalMode::Format { write, .. } => *write,
            };

            let printed = file_guard
                .format_file()
                .with_file_id_and_code(file_id, "Format")?;

            let output = printed.into_code();
            if output != input {
                if write {
                    file.set_content(output.as_bytes()).with_file_id(file_id)?;
                } else {
                    if !ctx.execution.should_report_to_terminal() {
                        ctx.push_format_stat(
                            path.display().to_string(),
                            FormatterReportFileDetail {
                                formatted_content: Some(output.clone()),
                            },
                        )
                    }
                    // Returning the diff message will discard the content of
                    // diagnostics, meaning those would not be printed so they
                    // have to be manually sent through the console channel
                    if let FileStatus::Message(msg) = result {
                        ctx.messages.send(msg).ok();
                    }

                    return Ok(FileStatus::Message(Message::Diff {
                        file_name: path.display().to_string(),
                        old: input,
                        new: output,
                    }));
                }
            }
        }

        Ok(result)
    })
}

/// Wrapper type for messages that can be printed during the traversal process
enum Message {
    SkippedFixes {
        /// Suggested fixes skipped during the lint traversal
        skipped_suggested_fixes: u32,
    },
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
