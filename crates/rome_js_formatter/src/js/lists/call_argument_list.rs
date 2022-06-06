use crate::generated::FormatJsCallArgumentList;
use crate::prelude::*;
use crate::utils::format_separated_for_call_arguments;
use rome_formatter::write;
use rome_js_syntax::{JsCallArgumentList, JsSyntaxKind};

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn fmt(node: &JsCallArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        let args = format_with(|f| {
            let separated = node
                .format_separated(JsSyntaxKind::COMMA)
                .with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Elide),
                )
                .map(|e| e.memoized());
            format_separated_for_call_arguments(separated, node.len(), f)
        });

        dbg_write!(f, [&group_elements(&soft_block_indent(&args))])
    }
}
