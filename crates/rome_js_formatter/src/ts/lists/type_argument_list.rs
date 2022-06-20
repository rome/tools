use crate::generated::FormatTsTypeArgumentList;
use crate::prelude::*;
use rome_js_syntax::{JsSyntaxKind, TsTypeArgumentList};

impl FormatRule<TsTypeArgumentList> for FormatTsTypeArgumentList {
    type Context = JsFormatContext;

    fn fmt(node: &TsTypeArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(JsSyntaxKind::COMMA)
                    .with_trailing_separator(TrailingSeparator::Disallowed)
                    .group_nodes(false),
            )
            .finish()
    }
}
