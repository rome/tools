use std::collections::HashMap;

use crate::{session::Session, utils};
use anyhow::Result;
use tower_lsp::lsp_types::{RenameParams, TextEdit, WorkspaceEdit};
use tracing::trace;

pub(crate) fn rename(session: &Session, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
    let url = params.text_document_position.text_document.uri;
    let rome_path = session.file_path(&url);

    trace!("Renaming...");

    let doc = session.document(&url)?;
    let cursor_range =
        crate::utils::offset(&doc.line_index, params.text_document_position.position);

    let result = session
        .workspace
        .rename(rome_service::workspace::RenameParams {
            path: rome_path,
            symbol_at: cursor_range,
            new_name: params.new_name,
        })?;

    let mut changes = HashMap::new();
    changes.insert(
        url,
        result
            .indels
            .into_iter()
            .map(|indel| TextEdit {
                range: utils::range(&doc.line_index, indel.delete),
                new_text: indel.insert,
            })
            .collect(),
    );

    let workspace_edit = WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(workspace_edit))
}
