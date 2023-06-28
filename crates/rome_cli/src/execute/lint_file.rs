use crate::execute::diagnostics::{ResultExt, ResultIoExt};
use crate::execute::process_file::{FileResult, FileStatus, Message};
use crate::execute::traverse::TraversalOptions;
use crate::CliDiagnostic;
use rome_diagnostics::{category, Error};
use rome_fs::{OpenOptions, RomePath};
use rome_service::file_handlers::Language;
use rome_service::workspace::{FileGuard, OpenFileParams, RuleCategories};
use std::path::Path;
use std::sync::atomic::Ordering;

pub(crate) struct LintFile<'ctx, 'app> {
    pub(crate) ctx: &'app TraversalOptions<'ctx, 'app>,
    pub(crate) path: &'app Path,
}

/// Lints a single file and returns a [FileResult]
pub(crate) fn lint_file(payload: LintFile) -> FileResult {
    let LintFile { ctx, path } = payload;
    let rome_path = RomePath::new(path);
    let mut errors = 0;
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
    if let Some(fix_mode) = ctx.execution.as_fix_file_mode() {
        let fixed = file_guard
            .fix_file(*fix_mode, false)
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

    let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
    let result = file_guard
        .pull_diagnostics(RuleCategories::LINT, max_diagnostics.into())
        .with_file_path_and_code(path.display().to_string(), category!("lint"))?;

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
    ctx.increment_processed();
    if errors > 0 {
        return Ok(FileStatus::Message(Message::ApplyError(
            CliDiagnostic::file_apply_error(path.display().to_string()),
        )));
    } else {
        Ok(result)
    }
}
