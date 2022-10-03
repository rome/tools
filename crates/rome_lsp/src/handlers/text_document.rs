use anyhow::{bail, Result};
use rome_service::workspace::{ChangeFileParams, CloseFileParams, Language, OpenFileParams};
use tower_lsp::lsp_types;
use tracing::error;

use crate::{documents::Document, session::Session};

/// Handler for `textDocument/didOpen` LSP notification
#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) async fn did_open(
    session: &Session,
    params: lsp_types::DidOpenTextDocumentParams,
) -> Result<()> {
    let url = params.text_document.uri;
    let version = params.text_document.version;
    let content = params.text_document.text;
    let language_hint = Language::from_language_id(&params.text_document.language_id);

    let rome_path = session.file_path(&url);
    let doc = Document::new(version, &content);

    session.workspace.open_file(OpenFileParams {
        path: rome_path,
        version,
        content,
        language_hint,
    })?;

    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Handler for `textDocument/didChange` LSP notification
#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) async fn did_change(
    session: &Session,
    params: lsp_types::DidChangeTextDocumentParams,
) -> Result<()> {
    let url = params.text_document.uri;
    let version = params.text_document.version;
    let rome_path = session.file_path(&url);

    // Because of TextDocumentSyncKind::Full, there should only be one change.
    let mut content_changes = params.content_changes;
    let content = match content_changes.pop() {
        Some(change) => change.text,
        None => bail!("Invalid textDocument/didChange for {:?}", url),
    };

    let doc = Document::new(version, &content);

    session.workspace.change_file(ChangeFileParams {
        path: rome_path,
        version,
        content,
    })?;

    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Handler for `textDocument/didClose` LSP notification
#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) async fn did_close(
    session: &Session,
    params: lsp_types::DidCloseTextDocumentParams,
) -> Result<()> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url);

    session
        .workspace
        .close_file(CloseFileParams { path: rome_path })?;

    session.remove_document(&url);

    let diagnostics = vec![];
    let version = None;
    session
        .client
        .publish_diagnostics(url, diagnostics, version)
        .await;

    Ok(())
}
