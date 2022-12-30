use crate::session::Session;
use crate::utils;
use anyhow::{Context, Error, Result};
use rome_rowan::TextRange;
use rome_service::{
    workspace::{FormatFileParams, FormatOnTypeParams, FormatRangeParams},
    WorkspaceError,
};
use tower_lsp::lsp_types::*;
use tracing::debug;

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format(
    session: &Session,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url)?;

    let doc = session.document(&url)?;

    debug!("Formatting...");
    let result = session
        .workspace
        .format_file(FormatFileParams { path: rome_path });

    let printed = match result {
        Ok(printed) => printed,
        Err(WorkspaceError.format_with_errors_disabled()) | Err(WorkspaceError::FileIgnored(_)) => {
            return Ok(None)
        }
        Err(err) => return Err(Error::from(err)),
    };

    let num_lines: u32 = doc.line_index.len();

    let range = Range {
        start: Position::default(),
        end: Position {
            line: num_lines,
            character: 0,
        },
    };

    let edits = vec![TextEdit {
        range,
        new_text: printed.into_code(),
    }];

    Ok(Some(edits))
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_range(
    session: &Session,
    params: DocumentRangeFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url)?;
    let doc = session.document(&url)?;

    let start_index = utils::offset(&doc.line_index, params.range.start).with_context(|| {
        format!(
            "failed to access position {:?} in document {url}",
            params.range.start
        )
    })?;
    let end_index = utils::offset(&doc.line_index, params.range.end).with_context(|| {
        format!(
            "failed to access position {:?} in document {url}",
            params.range.end
        )
    })?;

    let format_range = TextRange::new(start_index, end_index);
    let result = session.workspace.format_range(FormatRangeParams {
        path: rome_path,
        range: format_range,
    });

    let formatted = match result {
        Ok(formatted) => formatted,
        Err(WorkspaceError.format_with_errors_disabled()) | Err(WorkspaceError::FileIgnored(_)) => {
            return Ok(None)
        }
        Err(err) => return Err(Error::from(err)),
    };

    // Recalculate the actual range that was reformatted from the formatter result
    let formatted_range = match formatted.range() {
        Some(range) => {
            let start_loc = utils::position(&doc.line_index, range.start())?;
            let end_loc = utils::position(&doc.line_index, range.end())?;
            Range {
                start: start_loc,
                end: end_loc,
            }
        }
        None => Range {
            start: Position::default(),
            end: Position {
                line: doc.line_index.len(),
                character: 0,
            },
        },
    };

    Ok(Some(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }]))
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_on_type(
    session: &Session,
    params: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let url = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let rome_path = session.file_path(&url)?;
    let doc = session.document(&url)?;

    let offset = utils::offset(&doc.line_index, position)
        .with_context(|| format!("failed to access position {position:?} in document {url}"))?;

    let result = session.workspace.format_on_type(FormatOnTypeParams {
        path: rome_path,
        offset,
    });

    let formatted = match result {
        Ok(formatted) => formatted,
        Err(WorkspaceError.format_with_errors_disabled()) | Err(WorkspaceError::FileIgnored(_)) => {
            return Ok(None)
        }
        Err(err) => return Err(Error::from(err)),
    };

    // Recalculate the actual range that was reformatted from the formatter result
    let formatted_range = match formatted.range() {
        Some(range) => {
            let start_loc = utils::position(&doc.line_index, range.start())?;
            let end_loc = utils::position(&doc.line_index, range.end())?;
            Range {
                start: start_loc,
                end: end_loc,
            }
        }
        None => Range {
            start: Position::default(),
            end: Position {
                line: doc.line_index.len(),
                character: 0,
            },
        },
    };

    Ok(Some(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }]))
}
