use lsp_text::RopeExt;
use lspower::lsp;
use ropey::Rope;
use std::sync::Arc;
use tracing::error;

use crate::{documents::Document, session::Session};

pub async fn did_open(session: Arc<Session>, params: lsp::DidOpenTextDocumentParams) {
    let uri = params.text_document.uri.clone();
    let file_id = session.file_id(uri.clone());
    let version = params.text_document.version;

    let doc = Document::new(params.text_document.text, version, file_id);
    session.insert_document(uri.clone(), doc);

    if let Err(err) = session.update_diagnostics(uri).await {
        error!("Failed to update diagnostics: {}", err);
    }
}

pub async fn did_change(session: Arc<Session>, params: lsp::DidChangeTextDocumentParams) {
    let url = params.text_document.uri;
    let version = params.text_document.version;

    let doc = match session.document(&url) {
        Ok(doc) => doc,
        Err(err) => return error!("{}", err),
    };

    let mut content = Rope::from(doc.text.as_ref());

    // Edits must all be built before content is mutated
    let edits: Result<Vec<_>, _> = params
        .content_changes
        .iter()
        .map(|change| content.build_edit(change))
        .collect();

    match edits {
        Ok(edits) => edits.iter().for_each(|e| content.apply_edit(e)),
        Err(err) => return error!("{}", err),
    }

    let doc = Document::new(content.to_string(), version, doc.file_id);
    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }
}

pub async fn did_close(session: Arc<Session>, params: lsp::DidCloseTextDocumentParams) {
    let uri = params.text_document.uri;
    session.remove_document(&uri);
    let diagnostics = vec![];
    let version = None;
    session
        .client
        .publish_diagnostics(uri, diagnostics, version)
        .await;
}
