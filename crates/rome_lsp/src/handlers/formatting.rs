use crate::config::WorkspaceSettings;
use crate::line_index::{self, LineCol};
use anyhow::{bail, Result};
use lspower::lsp::*;
use rome_analyze::FileId;
use rome_formatter::{FormatOptions, IndentStyle};
use rslint_parser::{parse, SourceType, TextRange, TokenAtOffset};

/// Utility function that takes formatting options from [LSP](lspower::lsp::FormattingOptions)
/// and transforms that to [options](rome_formatter::FormatOptions) that the rome formatter can understand
pub(crate) fn to_format_options(params: &FormattingOptions) -> FormatOptions {
    let indent_style = if params.insert_spaces {
        IndentStyle::Space(params.tab_size as u8)
    } else {
        IndentStyle::Tab
    };
    FormatOptions {
        indent_style,
        ..FormatOptions::default()
    }
}

pub fn format(
    text: &str,
    file_id: FileId,
    params: &FormattingOptions,
    workspace_settings: WorkspaceSettings,
) -> Result<Option<Vec<TextEdit>>> {
    let syntax = SourceType::ts();

    let parse_result = parse(text, file_id, syntax);

    // can't format, we bail early
    if !workspace_settings.formatter.format_with_syntax_errors || parse_result.has_errors() {
        return Ok(None);
    }
    let options = to_format_options(params);

    let new_text = rome_formatter::format(options, &parse_result.syntax())?.into_code();

    let num_lines: u32 = line_index::LineIndex::new(text).newlines.len().try_into()?;

    let range = Range {
        start: Position::default(),
        end: Position {
            line: num_lines,
            character: 0,
        },
    };

    let edits = vec![TextEdit { range, new_text }];
    Ok(Some(edits))
}

pub(crate) struct FormatRangeParams<'input> {
    pub(crate) text: &'input str,
    pub(crate) file_id: FileId,
    pub(crate) range: Range,
    /// Options to pass to [rome_formatter]
    pub(crate) format_options: FormatOptions,
    pub(crate) workspace_settings: WorkspaceSettings,
}

pub(crate) fn format_range(params: FormatRangeParams) -> Result<Option<Vec<TextEdit>>> {
    let syntax = SourceType::ts();
    let parse_result = parse(params.text, params.file_id, syntax);

    // can't format, we bail early
    if params
        .workspace_settings
        .formatter
        .format_with_syntax_errors
        || !parse_result.has_errors()
    {
        return Ok(None);
    }
    let line_index = line_index::LineIndex::new(params.text);
    let start_index = line_index.offset(LineCol {
        line: params.range.start.line,
        col: params.range.start.character,
    });
    let end_index = line_index.offset(LineCol {
        line: params.range.end.line,
        col: params.range.end.character,
    });

    let tree = parse_result.syntax();
    let format_range = TextRange::new(start_index, end_index);
    let formatted = rome_formatter::format_range(params.format_options, &tree, format_range)?;

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

    Ok(Some(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }]))
}

pub(crate) struct FormatOnTypeParams<'input> {
    pub(crate) text: &'input str,
    pub(crate) file_id: FileId,
    pub(crate) position: Position,
    /// Options to pass to [rome_formatter]
    pub(crate) format_options: FormatOptions,
    pub(crate) workspace_settings: WorkspaceSettings,
}

pub(crate) fn format_on_type(params: FormatOnTypeParams) -> Result<Option<Vec<TextEdit>>> {
    let syntax = SourceType::ts();
    let parse_result = parse(params.text, params.file_id, syntax);

    // can't format, we bail early
    if params
        .workspace_settings
        .formatter
        .format_with_syntax_errors
        || !parse_result.has_errors()
    {
        return Ok(None);
    }
    let line_index = line_index::LineIndex::new(params.text);
    let offset = line_index.offset(LineCol {
        line: params.position.line,
        col: params.position.character,
    });
    let tree = parse_result.syntax();
    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => return Ok(None),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => bail!("found a token with no parent"),
    };

    let formatted = rome_formatter::format_node(params.format_options, &root_node)?;

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

    Ok(Some(vec![TextEdit {
        range: formatted_range,
        new_text: formatted.into_code(),
    }]))
}
