use rome_analyze::AnalysisFilter;
use rome_diagnostics::file::FileId;
use rome_js_parser::parse_script;
use rome_js_syntax::{JsAnyRoot, TextRange};
use tower_lsp::lsp_types::CodeActionOrCommand;
use tower_lsp::{jsonrpc, lsp_types};

use crate::line_index::LineIndex;
use crate::utils;

/// Queries the [`AnalysisServer`] for diagnostics of the file matching [`FileId`]
///
/// If the `AnalysisServer` has no matching file, results in error.
pub(crate) fn diagnostics(
    file_id: FileId,
    url: &lsp_types::Url,
    text: &str,
) -> jsonrpc::Result<Vec<lsp_types::Diagnostic>> {
    let parse = parse_script(text, file_id);
    let root = JsAnyRoot::from(parse.tree());

    let mut result = Vec::new();
    let line_index = LineIndex::new(text);

    rome_analyze::analyze(file_id, &root, AnalysisFilter::default(), |event| {
        let diag = event
            .diagnostic()
            .and_then(|d| utils::diagnostic_to_lsp(d, url, &line_index));

        if let Some(diag) = diag {
            result.push(diag);
        }
    });

    Ok(result)
}

/// Queries the [`AnalysisServer`] for code actions of the file matching [FileId]
///
/// If the AnalysisServer has no matching file, results in error.
pub(crate) fn code_actions(
    file_id: FileId,
    text: &str,
    url: lsp_types::Url,
    diagnostics: &[lsp_types::Diagnostic],
    cursor_range: TextRange,
) -> jsonrpc::Result<Vec<lsp_types::CodeActionOrCommand>> {
    let parse = parse_script(text, file_id);
    let root = JsAnyRoot::from(parse.tree());

    let filter = AnalysisFilter {
        range: Some(cursor_range),
        ..AnalysisFilter::default()
    };

    let mut has_fixes = false;
    let mut result = Vec::new();
    let line_index = LineIndex::new(text);

    rome_analyze::analyze(file_id, &root, filter, |event| {
        if let Some(action) = event.action() {
            let action = utils::code_fix_to_lsp(&url, &line_index, diagnostics, action);

            has_fixes |= action.diagnostics.is_some();
            result.push(CodeActionOrCommand::CodeAction(action));
        }
    });

    // If any actions is marked as fixing a diagnostic, hide other actions
    // that do not fix anything (refactor opportunities) to reduce noise
    if has_fixes {
        result.retain(|action| {
            if let CodeActionOrCommand::CodeAction(action) = action {
                action.diagnostics.is_some()
            } else {
                true
            }
        });
    }

    Ok(result)
}
