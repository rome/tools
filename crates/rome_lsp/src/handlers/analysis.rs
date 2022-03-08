use lspower::lsp::CodeActionOrCommand;
use lspower::{jsonrpc, lsp};
use rome_analyze::{AnalysisServer, FileId};
use rome_js_syntax::TextRange;

use crate::line_index::LineIndex;
use crate::utils;

/// Queries the [`AnalysisServer`] for diagnostics of the file matching [`FileId`]
///
/// If the `AnalysisServer` has no matching file, results in error.
pub(crate) fn diagnostics(
    analysis_server: AnalysisServer,
    file_id: FileId,
) -> jsonrpc::Result<Vec<lsp::Diagnostic>> {
    let text = analysis_server
        .get_file_text(file_id)
        .ok_or_else(jsonrpc::Error::internal_error)?;
    let line_index = LineIndex::new(&text);

    let diagnostics: Vec<_> = analysis_server
        .diagnostics(file_id)
        .filter_map(|d| utils::diagnostic_to_lsp(d.diagnostic, &line_index))
        .collect();
    Ok(diagnostics)
}

/// Queries the [`AnalysisServer`] for code actions of the file matching [FileId]
///
/// If the AnalysisServer has no matching file, results in error.
pub(crate) fn code_actions(
    analysis_server: AnalysisServer,
    file_id: FileId,
    url: lsp::Url,
    cursor_range: TextRange,
) -> jsonrpc::Result<Vec<lsp::CodeActionOrCommand>> {
    let text = analysis_server
        .get_file_text(file_id)
        .ok_or_else(jsonrpc::Error::internal_error)?;
    let line_index = LineIndex::new(&text);

    let code_actions: Vec<_> = analysis_server
        .actions(file_id, Some(cursor_range))
        .map(|a| utils::text_action_to_lsp(&a.into(), &line_index, url.to_owned(), None))
        .map(CodeActionOrCommand::CodeAction)
        .collect();

    Ok(code_actions)
}
