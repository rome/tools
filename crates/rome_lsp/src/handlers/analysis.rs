use anyhow::Result;
use rome_service::workspace::PullActionsParams;
use tower_lsp::lsp_types::{CodeActionOrCommand, CodeActionParams, CodeActionResponse};

use crate::session::Session;
use crate::utils;

/// Queries the [`AnalysisServer`] for code actions of the file matching [FileId]
///
/// If the AnalysisServer has no matching file, results in error.
pub(crate) fn code_actions(
    session: &Session,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>> {
    let workspace_settings = session.config.read().get_workspace_settings();
    if !workspace_settings.analysis.enable_code_actions {
        return Ok(Some(Vec::new()));
    }

    let url = params.text_document.uri.clone();
    let rome_path = session.file_path(&url);
    let doc = session.document(&url)?;

    let diagnostics = params.context.diagnostics;
    let cursor_range = crate::utils::text_range(&doc.line_index, params.range);

    let actions = session.workspace.pull_actions(PullActionsParams {
        path: rome_path,
        range: cursor_range,
    })?;

    let mut has_fixes = false;
    let mut actions: Vec<_> = actions
        .into_iter()
        .map(|action| {
            let action = utils::code_fix_to_lsp(&url, &doc.line_index, &diagnostics, action);
            has_fixes |= action.diagnostics.is_some();
            CodeActionOrCommand::CodeAction(action)
        })
        .collect();

    // If any actions is marked as fixing a diagnostic, hide other actions
    // that do not fix anything (refactor opportunities) to reduce noise
    if has_fixes {
        actions.retain(|action| {
            if let CodeActionOrCommand::CodeAction(action) = action {
                action.diagnostics.is_some()
            } else {
                true
            }
        });
    }

    Ok(Some(actions))
}
