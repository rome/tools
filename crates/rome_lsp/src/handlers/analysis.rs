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
    text: &str,
) -> jsonrpc::Result<Vec<lsp_types::Diagnostic>> {
    let parse = parse_script(text, file_id);
    let root = JsAnyRoot::from(parse.tree());

    let mut result = Vec::new();
    let line_index = LineIndex::new(text);

    rome_analyze::analyze(&root, AnalysisFilter::default(), |event| {
        if let Some(d) = event.diagnostic() {
            result.push(utils::diagnostic_to_lsp(d, &line_index));
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

    let mut result = Vec::new();
    let line_index = LineIndex::new(text);

    rome_analyze::analyze(&root, filter, |event| {
        if let Some(code_fix) = event.code_fix() {
            result.push(CodeActionOrCommand::CodeAction(utils::code_fix_to_lsp(
                &url,
                text,
                &line_index,
                diagnostics,
                code_fix,
            )));
        }
    });

    Ok(result)
}
