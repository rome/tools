use std::collections::HashMap;

use anyhow::{Context, Result};
use rome_analyze::ActionCategory;
use rome_fs::RomePath;
use rome_service::workspace::{
    FeatureName, FixFileMode, FixFileParams, PullActionsParams, SupportsFeatureParams,
};
use rome_service::RomeError;
use tower_lsp::lsp_types::{
    self as lsp, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
};

use crate::line_index::LineIndex;
use crate::session::Session;
use crate::utils;

const FIX_ALL: CodeActionKind = CodeActionKind::new("source.fixAll");

/// Queries the [`AnalysisServer`] for code actions of the file matching [FileId]
///
/// If the AnalysisServer has no matching file, results in error.
#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) fn code_actions(
    session: &Session,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>> {
    let url = params.text_document.uri.clone();
    let rome_path = session.file_path(&url);

    let linter_enabled = &session.workspace.supports_feature(SupportsFeatureParams {
        path: rome_path,
        feature: FeatureName::Lint,
    })?;
    if !linter_enabled {
        return Ok(Some(Vec::new()));
    }

    let mut has_fix_all = false;
    let mut filters = Vec::new();

    if let Some(filter) = params.context.only {
        for kind in filter {
            if kind == CodeActionKind::QUICKFIX {
                filters.push(ActionCategory::QuickFix);
            } else if kind == CodeActionKind::REFACTOR {
                filters.push(ActionCategory::Refactor);
            } else if kind == FIX_ALL {
                has_fix_all = true;
            } else {
                tracing::warn!("unknown code action kind {kind:?} requested");
            }
        }
    }

    let url = params.text_document.uri.clone();
    let rome_path = session.file_path(&url);
    let doc = session.document(&url)?;

    let diagnostics = params.context.diagnostics;
    let cursor_range = utils::text_range(&doc.line_index, params.range).with_context(|| {
        format!(
            "failed to access range {:?} in document {url}",
            params.range
        )
    })?;

    let result = session.workspace.pull_actions(PullActionsParams {
        path: rome_path.clone(),
        range: cursor_range,
    })?;

    // Generate an additional code action to apply all safe fixes on the
    // document if the action category "source.fixAll" was explicitly requested
    // by the language client
    let fix_all = if has_fix_all {
        fix_all(session, &url, rome_path, &doc.line_index, &diagnostics)?
    } else {
        None
    };

    let mut has_fixes = false;
    let mut actions: Vec<_> = result
        .actions
        .into_iter()
        .filter_map(|action| {
            // Remove actions that do not match the categories requested by the
            // language client
            if !filters.is_empty() && !filters.contains(&action.category) {
                return None;
            }

            let action = utils::code_fix_to_lsp(&url, &doc.line_index, &diagnostics, action);
            has_fixes |= action.diagnostics.is_some();

            Some(CodeActionOrCommand::CodeAction(action))
        })
        .chain(fix_all)
        .collect();

    // If any actions is marked as fixing a diagnostic, hide other actions
    // that do not fix anything (refactor opportunities) to reduce noise
    if has_fixes {
        actions.retain(|action| {
            if let CodeActionOrCommand::CodeAction(action) = action {
                action.kind == Some(FIX_ALL) || action.diagnostics.is_some()
            } else {
                true
            }
        });
    }

    Ok(Some(actions))
}

/// Generate a "fix all" code action for the given document
#[tracing::instrument(level = "trace", skip(session), err)]
fn fix_all(
    session: &Session,
    url: &lsp::Url,
    rome_path: RomePath,
    line_index: &LineIndex,
    diagnostics: &[lsp::Diagnostic],
) -> Result<Option<CodeActionOrCommand>, RomeError> {
    let fixed = session.workspace.fix_file(FixFileParams {
        path: rome_path,
        fix_file_mode: FixFileMode::SafeFixes,
    })?;

    if fixed.actions.is_empty() {
        return Ok(None);
    }

    let diagnostics = diagnostics
        .iter()
        .filter_map(|d| {
            let code = d.code.as_ref().and_then(|code| match code {
                lsp::NumberOrString::String(code) => Some(code.as_str()),
                lsp::NumberOrString::Number(_) => None,
            })?;

            let diag_range = utils::text_range(line_index, d.range).ok()?;
            let has_matching_rule = fixed.actions.iter().any(|action| {
                code == action.rule_name && action.range.intersect(diag_range).is_some()
            });

            if has_matching_rule {
                Some(d.clone())
            } else {
                None
            }
        })
        .collect();

    let mut changes = HashMap::new();
    changes.insert(
        url.clone(),
        vec![lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position::new(0, 0),
                end: lsp::Position::new(
                    line_index.newlines.len().try_into().unwrap_or(u32::MAX),
                    0,
                ),
            },
            new_text: fixed.code,
        }],
    );

    let edit = lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Fix all auto-fixable issues"),
        kind: Some(FIX_ALL),
        diagnostics: Some(diagnostics),
        edit: Some(edit),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    })))
}
