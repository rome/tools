use crate::execute::diagnostics::{ResultExt, ResultIoExt, SkippedDiagnostic, UnhandledDiagnostic};
use crate::execute::lint_file::{lint_file, LintFile};
use crate::execute::traverse::TraversalOptions;
use crate::execute::TraversalMode;
use crate::{CliDiagnostic, FormatterReportFileDetail};
use rome_diagnostics::{category, Context, DiagnosticExt, Error};
use rome_fs::{OpenOptions, RomePath};
use rome_service::workspace::{
    FeatureName, FeaturesBuilder, FileGuard, Language, OpenFileParams, RuleCategories, SupportKind,
    SupportsFeatureParams,
};
use std::path::Path;
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub(crate) enum FileStatus {
    Success,
    Message(Message),
    Ignored,
}

/// Wrapper type for messages that can be printed during the traversal process
#[derive(Debug)]
pub(crate) enum Message {
    SkippedFixes {
        /// Suggested fixes skipped during the lint traversal
        skipped_suggested_fixes: u32,
    },
    ApplyError(CliDiagnostic),
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
        diff_kind: DiffKind,
    },
}

#[derive(Debug)]
pub(crate) enum DiffKind {
    Format,
    OrganizeImports,
}

impl<D> From<D> for Message
where
    Error: From<D>,
    D: std::fmt::Debug,
{
    fn from(err: D) -> Self {
        Self::Error(Error::from(err))
    }
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
pub(crate) type FileResult = Result<FileStatus, Message>;

/// This function performs the actual processing: it reads the file from disk
/// and parse it; analyze and / or format it; then it either fails if error
/// diagnostics were emitted, or compare the formatted code with the original
/// content of the file and emit a diff or write the new content to the disk if
/// write mode is enabled
pub(crate) fn process_file(ctx: &TraversalOptions, path: &Path) -> FileResult {
    tracing::trace_span!("process_file", path = ?path).in_scope(move || {
        let rome_path = RomePath::new(path);

        let file_features = ctx
            .workspace
            .file_features(SupportsFeatureParams {
                path: rome_path.clone(),
                feature: FeaturesBuilder::new()
                    .with_formatter()
                    .with_linter()
                    .with_organize_imports()
                    .build(),
            })
            .with_file_path_and_code(
                path.display().to_string(),
                category!("files/missingHandler"),
            )?;

        let unsupported_reason = match ctx.execution.traversal_mode() {
            TraversalMode::Check { .. } => file_features
                .support_kind_for(&FeatureName::Lint)
                .and_then(|support_kind| {
                    if support_kind.is_not_enabled() {
                        Some(support_kind)
                    } else {
                        None
                    }
                })
                .and(
                    file_features
                        .support_kind_for(&FeatureName::Format)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                )
                .and(
                    file_features
                        .support_kind_for(&FeatureName::OrganizeImports)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                ),
            TraversalMode::CI { .. } => file_features
                .support_kind_for(&FeatureName::Lint)
                .and_then(|support_kind| {
                    if support_kind.is_not_enabled() {
                        Some(support_kind)
                    } else {
                        None
                    }
                })
                .and(
                    file_features
                        .support_kind_for(&FeatureName::Format)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                )
                .and(
                    file_features
                        .support_kind_for(&FeatureName::OrganizeImports)
                        .and_then(|support_kind| {
                            if support_kind.is_not_enabled() {
                                Some(support_kind)
                            } else {
                                None
                            }
                        }),
                ),
            TraversalMode::Format { .. } => file_features.support_kind_for(&FeatureName::Format),
            TraversalMode::Lint { .. } => file_features.support_kind_for(&FeatureName::Lint),
            TraversalMode::Migrate { .. } => None,
        };

        if let Some(reason) = unsupported_reason {
            match reason {
                SupportKind::FileNotSupported => {
                    return Err(Message::from(
                        UnhandledDiagnostic.with_file_path(path.display().to_string()),
                    ))
                }
                SupportKind::FeatureNotEnabled | SupportKind::Ignored => {
                    return Ok(FileStatus::Ignored)
                }
                SupportKind::Supported => {}
            };
        }

        // NOTE: this is a work in progress that will be refactored over time
        //
        // With time, we will create a separate file for each traversal mode. Reason to do so
        // is to keep the business logics of each traversal separate. Doing so would allow us to
        // lower the changes to break the business logic of other traversal.
        //
        // This would definitely repeat the code, but it's worth the effort in the long run.
        if let TraversalMode::Lint { .. } = ctx.execution.traversal_mode {
            // the unsupported case should be handled already at this point
            if file_features.supports_for(&FeatureName::Lint) {
                return lint_file(LintFile { ctx, path });
            }
        }

        let open_options = OpenOptions::default()
            .read(true)
            .write(ctx.execution.requires_write_access());
        let mut file = ctx
            .fs
            .open_with_options(path, open_options)
            .with_file_path(path.display().to_string())?;

        let mut input = String::new();
        file.read_to_string(&mut input)
            .with_file_path(path.display().to_string())?;
        ctx.increment_processed();

        let file_guard = FileGuard::open(
            ctx.workspace,
            OpenFileParams {
                path: rome_path,
                version: 0,
                content: input.clone(),
                language_hint: Language::default(),
            },
        )
        .with_file_path_and_code(path.display().to_string(), category!("internalError/fs"))?;

        let mut errors = 0;

        if let Some(fix_mode) = ctx.execution.as_fix_file_mode() {
            let fixed = file_guard
                .fix_file(*fix_mode, file_features.supports_for(&FeatureName::Format))
                .with_file_path_and_code(path.display().to_string(), category!("lint"))?;

            ctx.push_message(Message::SkippedFixes {
                skipped_suggested_fixes: fixed.skipped_suggested_fixes,
            });

            if fixed.code != input {
                file.set_content(fixed.code.as_bytes())
                    .with_file_path(path.display().to_string())?;
                file_guard.change_file(file.file_version(), fixed.code)?;
            }
            errors = fixed.errors;
        }
        if file_features.supports_for(&FeatureName::OrganizeImports) && ctx.execution.is_check() {
            let sorted = file_guard.organize_imports().with_file_path_and_code(
                path.display().to_string(),
                category!("internalError/fs"),
            )?;

            if sorted.code != input {
                if ctx.execution.is_check_apply_unsafe() {
                    file.set_content(sorted.code.as_bytes())
                        .with_file_path(path.display().to_string())?;
                    file_guard.change_file(file.file_version(), sorted.code)?;
                } else {
                    errors += 1;
                    ctx.messages
                        .send(Message::Diff {
                            file_name: path.display().to_string(),
                            old: input.clone(),
                            new: sorted.code,
                            diff_kind: DiffKind::OrganizeImports,
                        })
                        .ok();
                }
            }
        }

        // If we are here, errors were emitted when applying code actions, so checking only for errors should be safe
        if errors > 0 {
            return Ok(FileStatus::Message(Message::ApplyError(
                CliDiagnostic::file_apply_error(path.display().to_string()),
            )));
        } else if ctx.execution.is_check_apply() || ctx.execution.is_check_apply_unsafe() {
            return Ok(FileStatus::Success);
        }

        let categories =
            if ctx.execution.is_format() || !file_features.supports_for(&FeatureName::Lint) {
                RuleCategories::SYNTAX
            } else {
                RuleCategories::SYNTAX | RuleCategories::LINT
            };

        let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
        let result = file_guard
            .pull_diagnostics(categories, max_diagnostics.into())
            .with_file_path_and_code(path.display().to_string(), category!("lint"))?;

        // In formatting mode, abort immediately if the file has errors
        errors = result.errors;
        match ctx.execution.traversal_mode() {
            TraversalMode::Format { ignore_errors, .. } if errors > 0 => {
                return Err(if *ignore_errors {
                    Message::from(SkippedDiagnostic.with_file_path(path.display().to_string()))
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
        let no_diagnostics = result.diagnostics.is_empty() && result.skipped_diagnostics == 0;
        let result = if no_diagnostics || ctx.execution.is_format() {
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

        if file_features.supports_for(&FeatureName::OrganizeImports)
            // we want to print a diff only if we are in CI
            // or we are running "check" or "check --apply"
            && (ctx.execution.is_ci() || !ctx.execution.is_check_apply_unsafe())
        {
            let sorted = file_guard
                .organize_imports()
                .with_file_path(path.display().to_string())?;

            if sorted.code != input {
                ctx.messages
                    .send(Message::Diff {
                        file_name: path.display().to_string(),
                        old: input.clone(),
                        new: sorted.code,
                        diff_kind: DiffKind::OrganizeImports,
                    })
                    .ok();
            }
        }

        if file_features.supports_for(&FeatureName::Format) {
            let should_write = match ctx.execution.traversal_mode() {
                // In check mode do not run the formatter and return the result immediately,
                // but only if the argument `--apply` is not passed.
                TraversalMode::Check { .. } | TraversalMode::Lint { .. } => {
                    ctx.execution.as_fix_file_mode().is_some()
                }
                TraversalMode::CI { .. } => false,
                TraversalMode::Format { write, .. } => *write,
                TraversalMode::Migrate { write: dry_run, .. } => *dry_run,
            };

            let printed = file_guard
                .format_file()
                .with_file_path_and_code(path.display().to_string(), category!("format"))?;

            let output = printed.into_code();
            if output != input {
                if should_write {
                    file.set_content(output.as_bytes())
                        .with_file_path(path.display().to_string())?;
                    file_guard.change_file(file.file_version(), output)?;
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
                        diff_kind: DiffKind::Format,
                    }));
                }
            }
        }

        Ok(result)
    })
}
