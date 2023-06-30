use crate::converters::from_proto;
use crate::converters::line_index::LineIndex;
use crate::session::Session;
use crate::utils;
use anyhow::{Context, Result};
use rome_analyze::{ActionCategory, SourceActionKind};
use rome_fs::RomePath;
use rome_service::workspace::{
    FeatureName, FeaturesBuilder, FixFileMode, FixFileParams, PullActionsParams,
    SupportsFeatureParams,
};
use rome_service::WorkspaceError;
use std::borrow::Cow;
use std::collections::HashMap;
use tower_lsp::lsp_types::{
    self as lsp, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
};
use tracing::debug;

const FIX_ALL_CATEGORY: ActionCategory = ActionCategory::Source(SourceActionKind::FixAll);

fn fix_all_kind() -> CodeActionKind {
    match FIX_ALL_CATEGORY.to_str() {
        Cow::Borrowed(kind) => CodeActionKind::from(kind),
        Cow::Owned(kind) => CodeActionKind::from(kind),
    }
}
/// Queries the [`AnalysisServer`] for code actions of the file matching its path
///
/// If the AnalysisServer has no matching file, results in error.
#[tracing::instrument(level = "debug", skip_all, fields(uri = display(&params.text_document.uri), range = debug(params.range), only = debug(&params.context.only), diagnostics = debug(&params.context.diagnostics)), err)]
pub(crate) fn code_actions(
    session: &Session,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>> {
    let url = params.text_document.uri.clone();
    let rome_path = session.file_path(&url)?;

    let file_features = &session.workspace.file_features(SupportsFeatureParams {
        path: rome_path,
        feature: FeaturesBuilder::new()
            .with_linter()
            .with_organize_imports()
            .build(),
    })?;

    if !file_features.supports_for(&FeatureName::Lint)
        && !file_features.supports_for(&FeatureName::OrganizeImports)
    {
        debug!("Linter and organize imports are both disabled");
        return Ok(Some(Vec::new()));
    }

    let mut has_fix_all = false;
    let mut filters = Vec::new();

    if let Some(filter) = &params.context.only {
        for kind in filter {
            let kind = kind.as_str();
            if FIX_ALL_CATEGORY.matches(kind) {
                has_fix_all = true;
            }

            filters.push(kind);
        }
    }

    let url = params.text_document.uri.clone();
    let rome_path = session.file_path(&url)?;
    let doc = session.document(&url)?;
    let position_encoding = session.position_encoding();

    let diagnostics = params.context.diagnostics;
    let cursor_range = from_proto::text_range(&doc.line_index, params.range, position_encoding)
        .with_context(|| {
            format!(
                "failed to access range {:?} in document {url} {:?}",
                params.range, &doc.line_index,
            )
        })?;

    let result = match session.workspace.pull_actions(PullActionsParams {
        path: rome_path.clone(),
        range: cursor_range,
    }) {
        Ok(result) => result,
        Err(err) => {
            return if matches!(err, WorkspaceError::FileIgnored(_)) {
                Ok(Some(Vec::new()))
            } else {
                Err(err.into())
            }
        }
    };

    debug!("Pull actions result: {:?}", result);

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
            if action.category.matches("source.organizeImports.rome")
                && !file_features.supports_for(&FeatureName::OrganizeImports)
            {
                return None;
            }
            // Remove actions that do not match the categories requested by the
            // language client
            let matches_filters = filters.iter().any(|filter| action.category.matches(filter));
            if !filters.is_empty() && !matches_filters {
                return None;
            }

            let action = utils::code_fix_to_lsp(
                &url,
                &doc.line_index,
                position_encoding,
                &diagnostics,
                action,
            )
            .ok()?;

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
                action.kind.as_ref() == Some(&fix_all_kind()) || action.diagnostics.is_some()
            } else {
                true
            }
        });
    }

    debug!("Suggested actions: \n{:?}", &actions);

    Ok(Some(actions))
}

/// Generate a "fix all" code action for the given document
#[tracing::instrument(level = "debug", skip(session), err)]
fn fix_all(
    session: &Session,
    url: &lsp::Url,
    rome_path: RomePath,
    line_index: &LineIndex,
    diagnostics: &[lsp::Diagnostic],
) -> Result<Option<CodeActionOrCommand>, WorkspaceError> {
    let should_format = session
        .workspace
        .file_features(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: vec![FeatureName::Format],
        })?
        .supports_for(&FeatureName::Format);
    let fixed = session.workspace.fix_file(FixFileParams {
        path: rome_path,
        fix_file_mode: FixFileMode::SafeFixes,
        should_format,
    })?;

    if fixed.actions.is_empty() {
        return Ok(None);
    }

    let diagnostics = diagnostics
        .iter()
        .filter_map(|d| {
            let code = d.code.as_ref()?;
            let code = match code {
                lsp::NumberOrString::String(code) => code.as_str(),
                lsp::NumberOrString::Number(_) => return None,
            };

            let code = code.strip_prefix("lint/")?;
            let position_encoding = session.position_encoding();

            let diag_range = from_proto::text_range(line_index, d.range, position_encoding).ok()?;
            let has_matching_rule = fixed.actions.iter().any(|action| {
                let Some((group_name, rule_name)) = &action.rule_name else { return false };
                let Some(code) = code.strip_prefix(group_name.as_ref()) else { return false };
                let Some(code) = code.strip_prefix('/') else { return false };
                code == rule_name && action.range.intersect(diag_range).is_some()
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
                end: lsp::Position::new(line_index.len(), 0),
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
        kind: Some(fix_all_kind()),
        diagnostics: Some(diagnostics),
        edit: Some(edit),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    })))
}
