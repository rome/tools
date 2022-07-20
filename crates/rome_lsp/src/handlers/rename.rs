use std::collections::HashMap;

use crate::session::Session;
use anyhow::Result;
use tower_lsp::lsp_types::{Position, Range, RenameParams, TextEdit, WorkspaceEdit};
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

    let num_lines: u32 = doc.line_index.newlines.len().try_into()?;
    let range = Range {
        start: Position::default(),
        end: Position {
            line: num_lines,
            character: 0,
        },
    };

    let mut changes = HashMap::new();
    changes.insert(
        url,
        vec![TextEdit {
            range,
            new_text: result.code,
        }],
    );

    let workspace_edit = WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(workspace_edit))
}
