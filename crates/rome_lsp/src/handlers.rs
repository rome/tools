use anyhow::Result;
use lspower::lsp::*;
use rome_analyze::FileId;
use rome_formatter::{FormatOptions, Formatter, IndentStyle};
use rslint_parser::parse_text;

use crate::line_index;

pub fn format(text: &str, file_id: FileId) -> Result<Vec<TextEdit>> {
    let tree = parse_text(text, file_id).syntax();

    let options = FormatOptions {
        indent_style: IndentStyle::Tab,
        line_width: 80,
    };

    let new_text = Formatter::new(options)
        .format_root(&tree)
        // TODO: impl Error for FormatError
        .unwrap()
        .code()
        .to_string();

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
