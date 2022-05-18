use std::sync::Arc;

use rome_js_parser::symbols::ScopeResolutionEvent;
use tower_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Url};
use tracing::{debug, info};

use crate::{line_index::LineIndex, session::Session};

pub(crate) async fn goto_definition(
    session: Arc<Session>,
    params: GotoDefinitionParams,
) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
    info!("textDocument/definition request");
    debug!("textDocument/definition request: {:?}", params);

    let uri = params
        .text_document_position_params
        .text_document
        .uri
        .clone();
    let document = session.document(&uri)?;

    let text = &document.text;
    let line_indices = LineIndex::new(text);
    let symbols = parse_and_collect_symbols(document);
    let symbol = get_selected_symbol(params, &line_indices, &symbols);

    Ok(symbol
        .and_then(|symbol| to_same_file_location(&line_indices, uri, symbol))
        .map(GotoDefinitionResponse::Scalar))
}

fn to_same_file_location(
    line_indices: &LineIndex,
    uri: Url,
    symbol: &ScopeResolutionEvent,
) -> Option<Location> {
    match symbol {
        ScopeResolutionEvent::ReferenceFound {
            declared_at: Some(declared_at),
            ..
        } => Some(Location {
            uri,
            range: line_indices.to_lsp_range(declared_at),
        }),
        ScopeResolutionEvent::DeclarationFound { range, .. } => Some(Location {
            uri,
            range: line_indices.to_lsp_range(range),
        }),
        _ => None,
    }
}

fn get_selected_symbol<'a>(
    params: GotoDefinitionParams,
    line_indices: &LineIndex,
    symbols: &'a [ScopeResolutionEvent],
) -> Option<&'a ScopeResolutionEvent> {
    let offset = line_indices.offset(params.text_document_position_params.position.into());
    let symbol = symbols
        .iter()
        .find(|s| s.range().contains_inclusive(offset));
    symbol
}

fn parse_and_collect_symbols(document: crate::documents::Document) -> Vec<ScopeResolutionEvent> {
    let file_id = document.file_id();
    let source_type = document.get_source_type();
    let parse_result = rome_js_parser::parse(&document.text, file_id, source_type);
    tracing::debug_span!("symbols", file_id = file_id).in_scope(move || {
        rome_js_parser::symbols::symbols(parse_result.syntax()).collect::<Vec<_>>()
    })
}
