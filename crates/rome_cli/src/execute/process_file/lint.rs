use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use crate::CliDiagnostic;
use rome_diagnostics::{category, Error};
use rome_service::workspace::RuleCategories;
use std::path::Path;
use std::sync::atomic::Ordering;

/// Lints a single file and returns a [FileResult]
pub(crate) fn lint<'ctx>(ctx: &'ctx SharedTraversalOptions<'ctx, '_>, path: &Path) -> FileResult {
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    lint_with_guard(ctx, &mut workspace_file)
}

pub(crate) fn lint_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let mut errors = 0;
    let input = workspace_file.input()?;

    if let Some(fix_mode) = ctx.execution.as_fix_file_mode() {
        let fixed = workspace_file
            .guard()
            .fix_file(*fix_mode, false)
            .with_file_path_and_code(
                workspace_file.path.display().to_string(),
                category!("lint"),
            )?;

        ctx.push_message(Message::SkippedFixes {
            skipped_suggested_fixes: fixed.skipped_suggested_fixes,
        });

        if fixed.code != input {
            workspace_file.update_file(fixed.code)?;
        }
        errors = fixed.errors;
    }

    let max_diagnostics = ctx.remaining_diagnostics.load(Ordering::Relaxed);
    let result = workspace_file
        .guard()
        .pull_diagnostics(RuleCategories::LINT, max_diagnostics.into())
        .with_file_path_and_code(workspace_file.path.display().to_string(), category!("lint"))?;

    let no_diagnostics = result.diagnostics.is_empty() && result.skipped_diagnostics == 0;
    let result = if no_diagnostics || ctx.execution.is_format() {
        FileStatus::Success
    } else {
        FileStatus::Message(Message::Diagnostics {
            name: workspace_file.path.display().to_string(),
            content: input,
            diagnostics: result.diagnostics.into_iter().map(Error::from).collect(),
            skipped_diagnostics: result.skipped_diagnostics,
        })
    };

    if errors > 0 {
        return Ok(FileStatus::Message(Message::ApplyError(
            CliDiagnostic::file_apply_error(workspace_file.path.display().to_string()),
        )));
    } else {
        Ok(result)
    }
}
