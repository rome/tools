use anyhow::{bail, Result};
use lspower::lsp::*;
use rome_analyze::FileId;
use rome_formatter::{FormatOptions, IndentStyle};
use rslint_parser::{parse_script, TextRange, TokenAtOffset};

use crate::line_index::{self, LineCol};

pub fn format(text: &str, file_id: FileId) -> Result<Vec<TextEdit>> {
    let tree = parse_script(text, file_id).syntax();

    let options = FormatOptions {
        indent_style: IndentStyle::Tab,
        line_width: 80,
    };

    let new_text = rome_formatter::format(options, &tree)?.into_code();

    let num_lines: u32 = line_index::LineIndex::new(text).newlines.len().try_into()?;

    let range = Range {
        start: Position::default(),
        end: Position {
            line: num_lines,
            character: 0,
        },
    };

    let edits = vec![TextEdit { range, new_text }];
    Ok(edits)
}

pub(crate) struct FormatRangeParams<'input> {
    pub(crate) text: &'input str,
    pub(crate) file_id: FileId,
    pub(crate) indent_style: IndentStyle,
    pub(crate) range: Range,
}

pub(crate) fn format_range(params: FormatRangeParams) -> Result<Vec<TextEdit>> {
    let tree = parse_script(params.text, params.file_id).syntax();

    let line_index = line_index::LineIndex::new(params.text);
    let start_index = line_index.offset(LineCol {
        line: params.range.start.line,
        col: params.range.start.character,
    });
    let end_index = line_index.offset(LineCol {
        line: params.range.end.line,
        col: params.range.end.character,
    });

    let options = FormatOptions {
        indent_style: params.indent_style,
        line_width: 80,
    };

    let format_range = TextRange::new(start_index, end_index);
    let formatted = rome_formatter::format_range(options, &tree, format_range)?;

    // Recalculate the actual range that was reformatted from the formatter result
    let formatted_range = match formatted.range() {
        Some(range) => {
            let start_loc = line_index.line_col(range.start());
            let end_loc = line_index.line_col(range.end());
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
                line: line_index.newlines.len().try_into()?,
                character: 0,
            },
        },
    };

    Ok(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }])
}

pub(crate) struct FormatOnTypeParams<'input> {
    pub(crate) text: &'input str,
    pub(crate) file_id: FileId,
    pub(crate) indent_style: IndentStyle,
    pub(crate) position: Position,
}

pub(crate) fn format_on_type(params: FormatOnTypeParams) -> Result<Vec<TextEdit>> {
    let tree = parse_script(params.text, params.file_id).syntax();

    let line_index = line_index::LineIndex::new(params.text);
    let offset = line_index.offset(LineCol {
        line: params.position.line,
        col: params.position.character,
    });

    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => return Ok(vec![]),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => bail!("found a token with no parent"),
    };

    let options = FormatOptions {
        indent_style: params.indent_style,
        line_width: 80,
    };

    let formatted = rome_formatter::format_node(options, &root_node)?;

    // Recalculate the actual range that was reformatted from the formatter result
    let formatted_range = match formatted.range() {
        Some(range) => {
            let start_loc = line_index.line_col(range.start());
            let end_loc = line_index.line_col(range.end());
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
                line: line_index.newlines.len().try_into()?,
                character: 0,
            },
        },
    };

    Ok(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }])
}
