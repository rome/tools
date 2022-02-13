use lspower::{jsonrpc, lsp};
use rome_analyze::{AnalysisServer, FileId};
use rslint_parser::TextRange;

use crate::line_index::LineIndex;
use crate::session::into_lsp_error;
use crate::utils;

pub(crate) fn diagnostics(
    analysis_server: AnalysisServer,
    file_id: FileId,
) -> jsonrpc::Result<Vec<lsp::Diagnostic>> {
    let text = analysis_server
        .get_file_text(file_id)
        .map_err(into_lsp_error)?;
    let line_index = LineIndex::new(&text);

    let diagnostics: Vec<_> = analysis_server
        .diagnostics(file_id)
        .filter_map(|d| utils::diagnostic_to_lsp(d.diagnostic, &line_index))
        .collect();
    Ok(diagnostics)
}

pub(crate) fn code_actions(
    analysis_server: AnalysisServer,
    file_id: FileId,
    url: lsp::Url,
    cursor_range: TextRange,
) -> jsonrpc::Result<Vec<lsp::CodeAction>> {
    let text = analysis_server
        .get_file_text(file_id)
        .map_err(into_lsp_error)?;
    let line_index = LineIndex::new(&text);

    let code_actions: Vec<lsp::CodeAction> = analysis_server
        .actions(file_id, Some(cursor_range))
        .map(|a| utils::text_action_to_lsp(&a.into(), &line_index, url.to_owned(), None))
        .collect();

    Ok(code_actions)
}
