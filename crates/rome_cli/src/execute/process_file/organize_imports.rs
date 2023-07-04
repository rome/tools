use crate::execute::diagnostics::ResultExt;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{
    DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions,
};
use rome_diagnostics::category;
use std::path::Path;

/// Lints a single file and returns a [FileResult]
pub(crate) fn organize_imports_with_guard<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    workspace_file: &mut WorkspaceFile,
) -> FileResult {
    let sorted = workspace_file
        .guard()
        .organize_imports()
        .with_file_path_and_code(
            workspace_file.path.display().to_string(),
            category!("organizeImports"),
        )?;

    if sorted.code != workspace_file.input() {
        if ctx.execution.is_check_apply_unsafe() {
            workspace_file.update_file(sorted.code)?;
        } else {
            return Ok(FileStatus::Message(Message::Diff {
                file_name: workspace_file.path.display().to_string(),
                old: workspace_file.input().to_string(),
                new: sorted.code,
                diff_kind: DiffKind::OrganizeImports,
            }));
        }
    }

    Ok(FileStatus::Success)
}
