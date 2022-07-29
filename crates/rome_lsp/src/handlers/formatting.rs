use crate::session::Session;
use crate::utils;
use anyhow::{Context, Error, Result};
use rome_formatter::IndentStyle;
use rome_rowan::TextRange;
use rome_service::{
    workspace::{FormatFileParams, FormatOnTypeParams, FormatRangeParams},
    RomeError,
};
use tower_lsp::lsp_types::*;
use tracing::trace;

#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) fn format(
    session: &Session,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url);

    let doc = session.document(&url)?;

    let indent_style = if params.options.insert_spaces {
        IndentStyle::Space(params.options.tab_size as u8)
    } else {
        IndentStyle::Tab
    };

    trace!("Formatting...");
    let result = session.workspace.format_file(FormatFileParams {
        path: rome_path,
        indent_style,
    });

    let printed = match result {
        Ok(printed) => printed,
        Err(RomeError::FormatWithErrorsDisabled) => return Ok(None),
        Err(err) => return Err(Error::from(err)),
    };

    let num_lines: u32 = doc.line_index.newlines.len().try_into()?;

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

#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) fn format_range(
    session: &Session,
    params: DocumentRangeFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url);
    let doc = session.document(&url)?;

    let indent_style = if params.options.insert_spaces {
        IndentStyle::Space(params.options.tab_size as u8)
    } else {
        IndentStyle::Tab
    };

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
        indent_style,
    });

    let formatted = match result {
        Ok(formatted) => formatted,
        Err(RomeError::FormatWithErrorsDisabled) => return Ok(None),
        Err(err) => return Err(Error::from(err)),
    };

    // Recalculate the actual range that was reformatted from the formatter result
    let formatted_range = match formatted.range() {
        Some(range) => {
            let start_loc = doc.line_index.line_col(range.start());
            let end_loc = doc.line_index.line_col(range.end());
            Range {
                start: Position {
                    line: start_loc.line,
                    character: start_loc.col,
                },
                end: Position {
                    line: end_loc.line,
                    character: end_loc.col,
                },
            }
        }
        None => Range {
            start: Position::default(),
            end: Position {
                line: doc.line_index.newlines.len().try_into()?,
                character: 0,
            },
        },
    };

    Ok(Some(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }]))
}

#[tracing::instrument(level = "trace", skip(session), err)]
pub(crate) fn format_on_type(
    session: &Session,
    params: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let url = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let rome_path = session.file_path(&url);
    let doc = session.document(&url)?;

    let indent_style = if params.options.insert_spaces {
        IndentStyle::Space(params.options.tab_size as u8)
    } else {
        IndentStyle::Tab
    };

    let offset = utils::offset(&doc.line_index, position)
        .with_context(|| format!("failed to access position {position:?} in document {url}"))?;

    let result = session.workspace.format_on_type(FormatOnTypeParams {
        path: rome_path,
        offset,
        indent_style,
    });

    let formatted = match result {
        Ok(formatted) => formatted,
        Err(RomeError::FormatWithErrorsDisabled) => return Ok(None),
        Err(err) => return Err(Error::from(err)),
    };

    // Recalculate the actual range that was reformatted from the formatter result
    let formatted_range = match formatted.range() {
        Some(range) => {
            let start_loc = doc.line_index.line_col(range.start());
            let end_loc = doc.line_index.line_col(range.end());
            Range {
                start: Position {
                    line: start_loc.line,
                    character: start_loc.col,
                },
                end: Position {
                    line: end_loc.line,
                    character: end_loc.col,
                },
            }
        }
        None => Range {
            start: Position::default(),
            end: Position {
                line: doc.line_index.newlines.len().try_into()?,
                character: 0,
            },
        },
    };

    Ok(Some(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }]))
}
