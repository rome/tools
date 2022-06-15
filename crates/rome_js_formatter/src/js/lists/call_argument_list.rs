use crate::generated::FormatJsCallArgumentList;
use crate::prelude::*;
use crate::utils::fmt_arguments_multi_line;
use rome_formatter::write;
use rome_js_syntax::{JsCallArgumentList, JsSyntaxKind};

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn fmt(node: &JsCallArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        if node.len() == 0 {
            return Ok(());
        }

        write!(
            f,
            [&group_elements(&soft_block_indent(&format_with(|f| {
                let separated = node.format_separated(JsSyntaxKind::COMMA).with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Elide),
                );
                fmt_arguments_multi_line(separated, f)
            })))]
        )
    }
}
