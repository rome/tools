use super::process_file::{process_file, DiffKind, FileStatus, Message};
use crate::execute::diagnostics::{
    CIFormatDiffDiagnostic, CIOrganizeImportsDiffDiagnostic, ContentDiffAdvice,
    FormatDiffDiagnostic, OrganizeImportsDiffDiagnostic, PanicDiagnostic,
};
use crate::{
    CliDiagnostic, CliSession, Execution, FormatterReportFileDetail, FormatterReportSummary,
    Report, ReportDiagnostic, ReportDiff, ReportErrorKind, ReportKind, TraversalMode,
};
use crossbeam::{
    channel::{unbounded, Receiver, Sender},
    select,
};
use rome_console::{fmt, markup, Console, ConsoleExt};
use rome_diagnostics::{
    adapters::StdError, category, DiagnosticExt, Error, PrintDescription, PrintDiagnostic,
    Resource, Severity,
};
use rome_fs::{FileSystem, PathInterner, RomePath};
use rome_fs::{TraversalContext, TraversalScope};
use rome_service::workspace::{IsPathIgnoredParams, SupportsFeatureResult};
use rome_service::{
    workspace::{FeatureName, SupportsFeatureParams},
    Workspace, WorkspaceError,
};
use std::collections::HashSet;
use std::{
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

pub(crate) fn traverse(execution: Execution, mut session: CliSession) -> Result<(), CliDiagnostic> {
    init_thread_pool();

    let verbose = session.args.contains("--verbose");

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
                return Err(CliDiagnostic::unexpected_argument(
                    format!("{:?}", input),
                    execution.traversal_mode_subcommand(),
                ));
            }
        }
        inputs.push(input);
    }

    if inputs.is_empty() && execution.as_stdin_file().is_none() {
        return Err(CliDiagnostic::missing_argument(
            "<INPUT>",
            execution.traversal_mode_subcommand(),
        ));
    }

    let (interner, recv_files) = PathInterner::new();
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
                    verbose,
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

            TraversalMode::Migrate { write: false, .. } => {
                console.log(markup! {
                    <Info>"Checked your configuration file in "{duration}</Info>
                });
            }

            TraversalMode::Migrate { write: true, .. } => {
                console.log(markup! {
                    <Info>"Migrated your configuration file in "{duration}</Info>
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
    if count.saturating_sub(skipped) == 0 {
        Err(CliDiagnostic::no_files_processed())
    } else if errors > 0 {
        if execution.is_check_apply() {
            Err(CliDiagnostic::apply_error())
        } else {
            Err(CliDiagnostic::check_error())
        }
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
    recv_files: Receiver<PathBuf>,
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
    /// Whether the console thread should print diagnostics in verbose mode
    verbose: bool,
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
        verbose,
    } = options;

    let mut paths: HashSet<String> = HashSet::new();
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

            Message::ApplyError(error) => {
                *errors += 1;
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
                if mode.should_report_to_terminal() && should_print {
                    console.error(markup! {
                        {if verbose { PrintDiagnostic::verbose(&error) } else { PrintDiagnostic::simple(&error) }}
                    });
                }
            }

            Message::Error(mut err) => {
                let location = err.location();
                if let Some(Resource::File(file_path)) = location.resource.as_ref() {
                    // Retrieves the file name from the file ID cache, if it's a miss
                    // flush entries from the interner channel until it's found
                    let file_name = match paths.get(*file_path) {
                        Some(path) => Some(path),
                        None => loop {
                            match recv_files.recv() {
                                Ok(path) => {
                                    paths.insert(path.display().to_string());
                                    if path.display().to_string() == *file_path {
                                        break paths.get(&path.display().to_string());
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
                        err = err.with_file_path(path.as_str());
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
                            {if verbose { PrintDiagnostic::verbose(&err) } else { PrintDiagnostic::simple(&err) }}
                        });
                    }
                } else {
                    let location = err.location();
                    let path = match &location.resource {
                        Some(Resource::File(file)) => Some(*file),
                        _ => None,
                    };

                    let file_name = path.unwrap_or("<unknown>");
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
                            {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
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
                                    {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
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
                diff_kind,
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
                            match diff_kind {
                                DiffKind::Format => {
                                    let diag = CIFormatDiffDiagnostic {
                                        file_name: &file_name,
                                        diff: ContentDiffAdvice {
                                            old: &old,
                                            new: &new,
                                        },
                                    };
                                    console.error(markup! {
                                        {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
                                    });
                                }
                                DiffKind::OrganizeImports => {
                                    let diag = CIOrganizeImportsDiffDiagnostic {
                                        file_name: &file_name,
                                        diff: ContentDiffAdvice {
                                            old: &old,
                                            new: &new,
                                        },
                                    };
                                    console.error(markup! {
                                        {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
                                    });
                                }
                            };
                        } else {
                            match diff_kind {
                                DiffKind::Format => {
                                    let diag = FormatDiffDiagnostic {
                                        file_name: &file_name,
                                        diff: ContentDiffAdvice {
                                            old: &old,
                                            new: &new,
                                        },
                                    };
                                    console.error(markup! {
                                        {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
                                    });
                                }
                                DiffKind::OrganizeImports => {
                                    let diag = OrganizeImportsDiffDiagnostic {
                                        file_name: &file_name,
                                        diff: ContentDiffAdvice {
                                            old: &old,
                                            new: &new,
                                        },
                                    };
                                    console.error(markup! {
                                        {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
                                    });
                                }
                            };
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
            <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"rome check --apply-unsafe\n"</Emphasis></Info>
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
pub(crate) struct TraversalOptions<'ctx, 'app> {
    /// Shared instance of [FileSystem]
    pub(crate) fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    pub(crate) workspace: &'ctx dyn Workspace,
    /// Determines how the files should be processed
    pub(crate) execution: &'ctx Execution,
    /// File paths interner cache used by the filesystem traversal
    interner: PathInterner,
    /// Shared atomic counter storing the number of processed files
    processed: &'ctx AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: &'ctx AtomicUsize,
    /// Channel sending messages to the display thread
    pub(crate) messages: Sender<Message>,
    /// Channel sending reports to the reports thread
    sender_reports: Sender<ReportKind>,
    /// The approximate number of diagnostics the console will print before
    /// folding the rest into the "skipped diagnostics" counter
    pub(crate) remaining_diagnostics: &'ctx AtomicU16,
}

impl<'ctx, 'app> TraversalOptions<'ctx, 'app> {
    pub(crate) fn increment_processed(&self) {
        self.processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Send a message to the display thread
    pub(crate) fn push_message(&self, msg: impl Into<Message>) {
        self.messages.send(msg.into()).ok();
    }

    pub(crate) fn can_format(
        &self,
        rome_path: &RomePath,
    ) -> Result<SupportsFeatureResult, WorkspaceError> {
        self.workspace.supports_feature(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeatureName::Format,
        })
    }

    pub(crate) fn push_format_stat(&self, path: String, stat: FormatterReportFileDetail) {
        self.sender_reports
            .send(ReportKind::Formatter(path, stat))
            .ok();
    }

    pub(crate) fn can_lint(
        &self,
        rome_path: &RomePath,
    ) -> Result<SupportsFeatureResult, WorkspaceError> {
        self.workspace.supports_feature(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeatureName::Lint,
        })
    }

    pub(crate) fn can_organize_imports(
        &self,
        rome_path: &RomePath,
    ) -> Result<SupportsFeatureResult, WorkspaceError> {
        self.workspace.supports_feature(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeatureName::OrganizeImports,
        })
    }

    pub(crate) fn miss_handler_err(&self, err: WorkspaceError, rome_path: &RomePath) {
        self.push_diagnostic(
            StdError::from(err)
                .with_category(category!("files/missingHandler"))
                .with_file_path(rome_path.display().to_string()),
        );
    }
}

impl<'ctx, 'app> TraversalContext for TraversalOptions<'ctx, 'app> {
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, error: Error) {
        self.push_message(error);
    }

    fn can_handle(&self, rome_path: &RomePath) -> bool {
        if rome_path.is_dir() {
            let can_handle = !self
                .workspace
                .is_path_ignored(IsPathIgnoredParams {
                    rome_path: rome_path.clone(),
                    feature: self.execution.as_feature_name(),
                })
                .unwrap_or_else(|err| {
                    self.push_diagnostic(err.into());
                    false
                });
            return can_handle;
        }

        let can_lint = self.can_lint(rome_path);
        let can_format = self.can_format(rome_path);
        let can_organize_imports = self.can_organize_imports(rome_path);

        match self.execution.traversal_mode() {
            TraversalMode::Check { .. } => can_lint
                .map(|result| result.reason.is_none())
                .unwrap_or_else(|err| {
                    self.miss_handler_err(err, rome_path);
                    false
                }),
            TraversalMode::CI { .. } => match (can_format, can_lint, can_organize_imports) {
                // the result of the error is the same, rome can't handle the file
                (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => {
                    self.miss_handler_err(err, rome_path);
                    false
                }
                (Ok(can_format), Ok(can_lint), Ok(can_organize_imports)) => {
                    can_lint.reason.is_none()
                        || can_format.reason.is_none()
                        || can_organize_imports.reason.is_none()
                }
            },
            TraversalMode::Format { .. } => can_format
                .map(|result| result.reason.is_none())
                .unwrap_or_else(|err| {
                    self.miss_handler_err(err, rome_path);
                    false
                }),
            // Imagine if Rome can't handle its own configuration file...
            TraversalMode::Migrate { .. } => true,
        }
    }

    fn handle_file(&self, path: &Path) {
        handle_file(self, path)
    }
}

/// This function wraps the [process_file] function implementing the traversal
/// in a [catch_unwind] block and emit diagnostics in case of error (either the
/// traversal function returns Err or panics)
fn handle_file(ctx: &TraversalOptions, path: &Path) {
    match catch_unwind(move || process_file(ctx, path)) {
        Ok(Ok(FileStatus::Success)) => {}
        Ok(Ok(FileStatus::Message(msg))) => {
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

            ctx.push_message(
                PanicDiagnostic { message }.with_file_path(path.display().to_string()),
            );
        }
    }
}
