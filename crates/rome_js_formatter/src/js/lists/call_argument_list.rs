use crate::prelude::*;
use crate::utils::write_arguments_multi_line;
use rome_formatter::write;
use rome_js_syntax::{JsCallArgumentList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallArgumentList;

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsCallArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        if node.len() == 0 {
            return Ok(());
        }

        write!(
            f,
            [&group_elements(&soft_block_indent(&format_with(|f| {
                let separated = node.format_separated(JsSyntaxKind::COMMA).with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Omit),
                );
                write_arguments_multi_line(separated, f)
            })))]
        )
    }
}
