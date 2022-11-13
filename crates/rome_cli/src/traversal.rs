use crate::{
    CliSession, Execution, FormatterReportFileDetail, FormatterReportSummary, Report,
    ReportDiagnostic, ReportDiff, ReportErrorKind, ReportKind, Termination, TraversalMode,
};
use crossbeam::{
    channel::{unbounded, Receiver, Sender},
    select,
};
use rome_console::{fmt, markup, Console, ConsoleExt};
use rome_diagnostics::{
    file::FileId,
    v2::{
        self,
        adapters::{IoError, StdError},
        category, Advices, Category, DiagnosticExt, Error, FilePath, PrintDescription,
        PrintDiagnostic, Severity, Visit,
    },
};
use rome_fs::{FileSystem, IndexSetInterner, OpenOptions, PathInterner, RomePath};
use rome_fs::{TraversalContext, TraversalScope};
use rome_service::workspace::{SupportsFeatureResult, UnsupportedReason};
use rome_service::{
    workspace::{
        FeatureName, FileGuard, Language, OpenFileParams, RuleCategories, SupportsFeatureParams,
    },
    RomeError, Workspace,
};
use rome_text_edit::TextEdit;
use std::{
    collections::HashMap,
    ffi::OsString,
    io,
    panic::catch_unwind,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU16, AtomicUsize, Ordering},
        Once,
    },
    thread,
    time::{Duration, Instant},
};

struct CheckResult {
    count: usize,
    duration: Duration,
    errors: usize,
}
impl fmt::Display for CheckResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> io::Result<()> {
        markup!(<Info>"Checked "{self.count}" file(s) in "{self.duration}</Info>).fmt(fmt)?;

        if self.errors > 0 {
            markup!("\n"<Error>"Found "{self.errors}" error(s)"</Error>).fmt(fmt)?
        }
        Ok(())
    }
}

pub(crate) fn traverse(execution: Execution, mut session: CliSession) -> Result<(), Termination> {
    init_thread_pool();

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
                return Err(Termination::UnexpectedArgument {
                    subcommand: execution.traversal_mode_subcommand(),
                    argument: input,
                });
            }
        }
        inputs.push(input);
    }

    if inputs.is_empty() && execution.as_stdin_file().is_none() {
        return Err(Termination::MissingArgument {
            subcommand: execution.traversal_mode_subcommand(),
            argument: "<INPUT>",
        });
    }

    let (interner, recv_files) = IndexSetInterner::new();
    let (send_msgs, recv_msgs) = unbounded();
    let (sender_reports, recv_reports) = unbounded();

    let processed = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    let fs = &*session.app.fs;
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;

    let max_diagnostics = execution.get_max_diagnostics();
    let remaining_diagnostics = AtomicU16::new(max_diagnostics);

    let mut errors: usize = 0;
    let mut report = Report::default();

    let duration = thread::scope(|s| {
        thread::Builder::new()
            .name(String::from("rome::console"))
            .spawn_scoped(s, || {
                process_messages(ProcessMessagesOptions {
                    execution: &execution,
                    console,
                    recv_reports,
                    recv_files,
                    recv_msgs,
                    max_diagnostics,
                    remaining_diagnostics: &remaining_diagnostics,
                    errors: &mut errors,
                    report: &mut report,
                });
            })
            .expect("failed to spawn console thread");

        // The traversal context is scoped to ensure all the channels it
        // contains are properly closed once the traversal finishes
        traverse_inputs(
            fs,
            inputs,
            &TraversalOptions {
                fs,
                workspace,
                execution: &execution,
                interner,
                processed: &processed,
                skipped: &skipped,
                messages: send_msgs,
                sender_reports,
                remaining_diagnostics: &remaining_diagnostics,
            },
        )
    });

    let count = processed.load(Ordering::Relaxed);
    let skipped = skipped.load(Ordering::Relaxed);

    if execution.should_report_to_terminal() {
        match execution.traversal_mode() {
            TraversalMode::Check { .. } => {
                if execution.as_fix_file_mode().is_some() {
                    console.log(markup! {
                        <Info>"Fixed "{count}" file(s) in "{duration}</Info>
                    });
                } else {
                    console.log(markup!({
                        CheckResult {
                            count,
                            duration,
                            errors,
                        }
                    }));
                }
            }
            TraversalMode::CI { .. } => {
                console.log(markup!({
                    CheckResult {
                        count,
                        duration,
                        errors,
                    }
                }));
            }
            TraversalMode::Format { write: false, .. } => {
                console.log(markup! {
                    <Info>"Compared "{count}" file(s) in "{duration}</Info>
                });
            }
            TraversalMode::Format { write: true, .. } => {
                console.log(markup! {
                    <Info>"Formatted "{count}" file(s) in "{duration}</Info>
                });
            }
        }
    } else {
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

    if skipped > 0 {
        console.log(markup! {
            <Warn>"Skipped "{skipped}" file(s)"</Warn>
        });
    }

    // Processing emitted error diagnostics, exit with a non-zero code
    if errors > 0 {
        Err(Termination::CheckError)
    } else {
        Ok(())
    }
}

/// This function will setup the global Rayon thread pool the first time it's called
///
/// This is currently only used to assign friendly debug names to the threads of the pool
fn init_thread_pool() {
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("rome::worker_{index}"))
            .build_global()
            .expect("failed to initialize the global thread pool");
    });
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
    execution: &'ctx Execution,
    /// Mutable reference to the [console](Console)
    console: &'ctx mut dyn Console,
    /// Receiver channel for reporting statistics
    recv_reports: Receiver<ReportKind>,
    /// Receiver channel that expects info when a file is processed
    recv_files: Receiver<(FileId, PathBuf)>,
    /// Receiver channel that expects info when a message is sent
    recv_msgs: Receiver<Message>,
    /// The maximum number of diagnostics the console thread is allowed to print
    max_diagnostics: u16,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    remaining_diagnostics: &'ctx AtomicU16,
    /// Mutable reference to a boolean flag tracking whether the console thread
    /// printed any error-level message
    errors: &'ctx mut usize,
    /// Mutable handle to a [Report] instance the console thread should write
    /// stats into
    report: &'ctx mut Report,
}

#[derive(Debug, v2::Diagnostic)]
#[diagnostic(
    category = "format",
    message = "File content differs from formatting output"
)]
struct CIDiffDiagnostic<'a> {
    #[location(resource)]
    file_name: &'a str,
    #[advice]
    diff: FormatDiffAdvice<'a>,
}

#[derive(Debug, v2::Diagnostic)]
#[diagnostic(
    severity = Information,
    category = "format",
    message = "Formatter would have printed the following content:"
)]
struct FormatDiffDiagnostic<'a> {
    #[location(resource)]
    file_name: &'a str,
    #[advice]
    diff: FormatDiffAdvice<'a>,
}

#[derive(Debug)]
struct FormatDiffAdvice<'a> {
    old: &'a str,
    new: &'a str,
}

impl Advices for FormatDiffAdvice<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let diff = TextEdit::from_unicode_words(self.old, self.new);
        visitor.record_diff(&diff)
    }
}

#[derive(Debug, v2::Diagnostic)]
struct TraversalDiagnostic<'a> {
    #[location(resource)]
    file_name: Option<&'a str>,
    #[severity]
    severity: v2::Severity,
    #[category]
    category: &'static Category,
    #[message]
    #[description]
    message: &'a str,
}

/// This thread receives [Message]s from the workers through the `recv_msgs`
/// and `recv_files` channels and handles them based on [Execution]
fn process_messages(options: ProcessMessagesOptions) {
    let ProcessMessagesOptions {
        execution: mode,
        console,
        recv_reports,
        recv_files,
        recv_msgs,
        max_diagnostics,
        remaining_diagnostics,
        errors,
        report,
    } = options;

    let mut paths = HashMap::new();
    let mut printed_diagnostics: u16 = 0;
    let mut not_printed_diagnostics = 0;
    let mut total_skipped_suggested_fixes = 0;

    let mut is_msg_open = true;
    let mut is_report_open = true;

    while is_msg_open || is_report_open {
        let msg = select! {
            recv(recv_msgs) -> msg => match msg {
                Ok(msg) => msg,
                Err(_) => {
                    is_msg_open = false;
                    continue;
                },
            },
            recv(recv_reports) -> stat => {
                match stat {
                    Ok(stat) => {
                        report.push_detail_report(stat);
                    }
                    Err(_) => {
                        is_report_open = false;
                    },
                }
                continue;
            }
        };

        match msg {
            Message::SkippedFixes {
                skipped_suggested_fixes,
            } => {
                total_skipped_suggested_fixes += skipped_suggested_fixes;
            }

            Message::Error(mut err) => {
                if let Some(location) = err.location() {
                    if let v2::Resource::File(FilePath::FileId(file_id)) = location.resource {
                        // Retrieves the file name from the file ID cache, if it's a miss
                        // flush entries from the interner channel until it's found
                        let file_name = match paths.get(&file_id) {
                            Some(path) => Some(path),
                            None => loop {
                                match recv_files.recv() {
                                    Ok((id, path)) => {
                                        paths.insert(id, path.display().to_string());
                                        if id == file_id {
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

                        if let Some(path) = file_name {
                            err = err.with_file_path(FilePath::PathAndId {
                                path: path.as_str(),
                                file_id,
                            });
                        }
                    }
                }

                let should_print = printed_diagnostics < max_diagnostics;
                if should_print {
                    printed_diagnostics += 1;
                    remaining_diagnostics.store(
                        max_diagnostics.saturating_sub(printed_diagnostics),
                        Ordering::Relaxed,
                    );
                } else {
                    not_printed_diagnostics += 1;
                }

                if mode.should_report_to_terminal() {
                    if should_print {
                        console.error(markup! {
                            {PrintDiagnostic(&err)}
                        });
                    }
                } else {
                    let file_name = err
                        .location()
                        .and_then(|location| {
                            let path = match &location.resource {
                                v2::Resource::File(file) => file,
                                _ => return None,
                            };

                            path.path()
                        })
                        .unwrap_or("<unknown>");

                    let title = PrintDescription(&err).to_string();
                    let code = err.category().and_then(|code| code.name().parse().ok());

                    report.push_detail_report(ReportKind::Error(
                        file_name.to_string(),
                        ReportErrorKind::Diagnostic(ReportDiagnostic {
                            code,
                            title,
                            severity: err.severity(),
                        }),
                    ));
                }
            }

            Message::Diagnostics {
                name,
                content,
                diagnostics,
                skipped_diagnostics,
            } => {
                not_printed_diagnostics += skipped_diagnostics;

                // is CI mode we want to print all the diagnostics
                if mode.is_ci() {
                    for diag in diagnostics {
                        if diag.severity() == Severity::Error {
                            *errors += 1;
                        }

                        let diag = diag.with_file_path(&name).with_file_source_code(&content);
                        console.error(markup! {
                            {PrintDiagnostic(&diag)}
                        });
                    }
                } else {
                    for diag in diagnostics {
                        let severity = diag.severity();
                        if severity == Severity::Error {
                            *errors += 1;
                        }

                        let should_print = printed_diagnostics < max_diagnostics;
                        if should_print {
                            printed_diagnostics += 1;
                            remaining_diagnostics.store(
                                max_diagnostics.saturating_sub(printed_diagnostics),
                                Ordering::Relaxed,
                            );
                        } else {
                            not_printed_diagnostics += 1;
                        }

                        if mode.should_report_to_terminal() {
                            if should_print {
                                let diag =
                                    diag.with_file_path(&name).with_file_source_code(&content);
                                console.error(markup! {
                                    {PrintDiagnostic(&diag)}
                                });
                            }
                        } else {
                            report.push_detail_report(ReportKind::Error(
                                name.to_string(),
                                ReportErrorKind::Diagnostic(ReportDiagnostic {
                                    code: diag.category().and_then(|code| code.name().parse().ok()),
                                    title: String::from("test here"),
                                    severity,
                                }),
                            ));
                        }
                    }
                }
            }

            Message::Diff {
                file_name,
                old,
                new,
            } => {
                if mode.is_ci() {
                    // A diff is an error in CI mode
                    *errors += 1;
                }

                let should_print = printed_diagnostics < max_diagnostics;
                if should_print {
                    printed_diagnostics += 1;
                    remaining_diagnostics.store(
                        max_diagnostics.saturating_sub(printed_diagnostics),
                        Ordering::Relaxed,
                    );
                } else {
                    not_printed_diagnostics += 1;
                }

                if mode.should_report_to_terminal() {
                    if should_print {
                        if mode.is_ci() {
                            let diag = CIDiffDiagnostic {
                                file_name: &file_name,
                                diff: FormatDiffAdvice {
                                    old: &old,
                                    new: &new,
                                },
                            };

                            console.error(markup! {
                                {PrintDiagnostic(&diag)}
                            });
                        } else {
                            let diag = FormatDiffDiagnostic {
                                file_name: &file_name,
                                diff: FormatDiffAdvice {
                                    old: &old,
                                    new: &new,
                                },
                            };

                            console.error(markup! {
                                {PrintDiagnostic(&diag)}
                            });
                        }
                    }
                } else {
                    report.push_detail_report(ReportKind::Error(
                        file_name,
                        ReportErrorKind::Diff(ReportDiff {
                            before: old,
                            after: new,
                            severity: Severity::Error,
                        }),
                    ));
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
}

/// Context object shared between directory traversal tasks
struct TraversalOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    workspace: &'ctx dyn Workspace,
    /// Determines how the files should be processed
    execution: &'ctx Execution,
    /// File paths interner used by the filesystem traversal
    interner: IndexSetInterner,
    /// Shared atomic counter storing the number of processed files
    processed: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    messages: Sender<Message>,
    /// Channel sending reports to the reports thread
    sender_reports: Sender<ReportKind>,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    remaining_diagnostics: &'ctx AtomicU16,
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

    fn miss_handler_err(&self, err: RomeError, rome_path: &RomePath) {
        self.push_diagnostic(
            StdError::from(err)
                .with_category(category!("files/missingHandler"))
                .with_file_path(rome_path.file_id()),
        );
    }
}

impl<'ctx, 'app> TraversalContext for TraversalOptions<'ctx, 'app> {
    fn interner(&self) -> &dyn PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, error: Error) {
        self.push_message(error);
    }

    fn can_handle(&self, rome_path: &RomePath) -> bool {
        let can_lint = self.can_lint(rome_path);
        let can_format = self.can_format(rome_path);

        match self.execution.traversal_mode() {
            TraversalMode::Check { .. } => can_lint
                .map(|result| result.reason.is_none())
                .unwrap_or_else(|err| {
                    self.miss_handler_err(err, rome_path);
                    false
                }),
            TraversalMode::CI { .. } => match (can_format, can_lint) {
                // the result of the error is the same, rome can't handle the file
                (Err(err), _) | (_, Err(err)) => {
                    self.miss_handler_err(err, rome_path);
                    false
                }
                (Ok(can_format), Ok(can_lint)) => {
                    can_lint.reason.is_none() || can_format.reason.is_none()
                }
            },
            TraversalMode::Format { .. } => can_format
                .map(|result| result.reason.is_none())
                .unwrap_or_else(|err| {
                    self.miss_handler_err(err, rome_path);
                    false
                }),
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

            ctx.push_message(PanicDiagnostic { message }.with_file_path(file_id));
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
/// diagnostics were emitted, or compare the formatted code with the original
/// content of the file and emit a diff or write the new content to the disk if
/// write mode is enabled
fn process_file(ctx: &TraversalOptions, path: &Path, file_id: FileId) -> FileResult {
    tracing::trace_span!("process_file", path = ?path).in_scope(move || {
        let rome_path = RomePath::new(path, file_id);
        let supported_format = ctx
            .can_format(&rome_path)
            .with_file_id_and_code(file_id, category!("files/missingHandler"))?;
        let supported_lint = ctx
            .can_lint(&rome_path)
            .with_file_id_and_code(file_id, category!("files/missingHandler"))?;
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
                UnsupportedReason::FileNotSupported => {
                    Err(Message::from(UnhandledDiagnostic.with_file_path(file_id)))
                }
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
        .with_file_id_and_code(file_id, category!("internalError/fs"))?;

        if let Some(fix_mode) = ctx.execution.as_fix_file_mode() {
            let fixed = file_guard
                .fix_file(*fix_mode)
                .with_file_id_and_code(file_id, category!("lint"))?;

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

        let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
        let result = file_guard
            .pull_diagnostics(categories, max_diagnostics.into())
            .with_file_id_and_code(file_id, category!("lint"))?;

        // In formatting mode, abort immediately if the file has errors
        let errors = result.errors;
        match ctx.execution.traversal_mode() {
            TraversalMode::Format { ignore_errors, .. } if errors > 0 => {
                return Err(if *ignore_errors {
                    Message::from(SkippedDiagnostic.with_file_path(file_id))
                } else {
                    Message::Diagnostics {
                        name: path.display().to_string(),
                        content: input,
                        diagnostics: result.diagnostics.into_iter().map(Error::from).collect(),
                        skipped_diagnostics: result.skipped_diagnostics,
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
                diagnostics: result.diagnostics.into_iter().map(Error::from).collect(),
                skipped_diagnostics: result.skipped_diagnostics,
            })
        };

        if errors > 0 {
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
                .with_file_id_and_code(file_id, category!("format"))?;

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
    Error(Error),
    Diagnostics {
        name: String,
        content: String,
        diagnostics: Vec<Error>,
        skipped_diagnostics: u64,
    },
    Diff {
        file_name: String,
        old: String,
        new: String,
    },
}

impl<D> From<D> for Message
where
    Error: From<D>,
{
    fn from(err: D) -> Self {
        Self::Error(Error::from(err))
    }
}

#[derive(Debug, v2::Diagnostic)]
#[diagnostic(category = "internalError/panic", tags(INTERNAL))]
struct PanicDiagnostic {
    #[description]
    #[message]
    message: String,
}

#[derive(Debug, v2::Diagnostic)]
#[diagnostic(category = "files/missingHandler", message = "unhandled file type")]
struct UnhandledDiagnostic;

#[derive(Debug, v2::Diagnostic)]
#[diagnostic(category = "parse", message = "Skipped file with syntax errors")]
struct SkippedDiagnostic;

/// Extension trait for turning [Display]-able error types into [TraversalError]
trait ResultExt {
    type Result;
    fn with_file_id_and_code(
        self,
        file_id: FileId,
        code: &'static Category,
    ) -> Result<Self::Result, Error>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    type Result = T;

    fn with_file_id_and_code(
        self,
        file_id: FileId,
        code: &'static Category,
    ) -> Result<Self::Result, Error> {
        self.map_err(move |err| {
            StdError::from(err)
                .with_category(code)
                .with_file_path(file_id)
        })
    }
}

/// Extension trait for turning [io::Error] into [Error]
trait ResultIoExt: ResultExt {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, Error>;
}

impl<T> ResultIoExt for io::Result<T> {
    fn with_file_id(self, file_id: FileId) -> Result<Self::Result, Error> {
        self.map_err(|error| IoError::from(error).with_file_path(file_id))
    }
}
