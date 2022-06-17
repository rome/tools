use crate::generated::FormatTsTypeParameterList;
use crate::prelude::*;
use rome_js_syntax::{JsSyntaxKind, TsTypeParameterList};
use rome_rowan::AstSeparatedList;

impl FormatRule<TsTypeParameterList> for FormatTsTypeParameterList {
    type Context = JsFormatContext;

    fn fmt(node: &TsTypeParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        // nodes and formatter are not aware of the source type (TSX vs TS), which means we can't
        // exactly pin point the exact case.
        //
        // This is just an heuristic to avoid removing the trailing comma from a TSX grammar.
        // This means that, if we are in a TS context and we have a trailing comma, the formatter won't remove it.
        // It's an edge case, while waiting for a better solution,
        let trailing_separator = if node.len() == 1 && node.trailing_separator().is_some() {
            TrailingSeparator::Mandatory
        } else {
            TrailingSeparator::default()
        };

        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA).with_options(
                FormatSeparatedOptions::default().with_trailing_separator(trailing_separator),
            ))
            .finish()
    }
}
