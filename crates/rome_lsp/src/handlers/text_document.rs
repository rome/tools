use lspower::lsp;
use rome_path::RomePath;
use std::sync::Arc;
use tracing::error;

use crate::{documents::Document, session::Session};

/// Handler for `textDocument/didOpen` LSP notification
pub(crate) async fn did_open(session: Arc<Session>, params: lsp::DidOpenTextDocumentParams) {
    let url = params.text_document.uri.clone();
    let file_id = session.file_id(url.clone());
    let version = params.text_document.version;
    let language_id = match params.text_document.language_id.as_str().try_into() {
        Ok(id) => id,
        Err(err) => return error!("{}", err),
    };
    let path = RomePath::new(url.path()).with_id(file_id);

    let doc = Document::new(path, language_id, version, params.text_document.text);
    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }
}

/// Handler for `textDocument/didChange` LSP notification
pub(crate) async fn did_change(session: Arc<Session>, params: lsp::DidChangeTextDocumentParams) {
    let url = params.text_document.uri;
    let version = params.text_document.version;

    let doc = match session.document(&url) {
        Ok(doc) => doc,
        Err(err) => return error!("{}", err),
    };

    // Because of TextDocumentSyncKind::Full, there should only be one change.
    let mut content_changes = params.content_changes;
    let text = match content_changes.pop() {
        Some(change) => change.text,
        None => return error!("Invalid textDocument/didChange for {:?}", url),
    };
    let path = RomePath::new(url.path()).with_id(doc.path.file_id().unwrap_or(0_usize));
    let doc = Document::new(path, doc.editor_language, version, text);
    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }
}

/// Handler for `textDocument/didClose` LSP notification
pub(crate) async fn did_close(session: Arc<Session>, params: lsp::DidCloseTextDocumentParams) {
    let url = params.text_document.uri;
    session.remove_document(&url);
    let diagnostics = vec![];
    let version = None;
    session
        .client
        .publish_diagnostics(url, diagnostics, version)
        .await;
}
