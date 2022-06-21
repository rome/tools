use crate::prelude::*;
use rome_js_syntax::{JsSyntaxKind, TsTupleTypeElementList};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTupleTypeElementList;

impl FormatRule<TsTupleTypeElementList> for FormatTsTupleTypeElementList {
    type Context = JsFormatContext;

    fn fmt(node: &TsTupleTypeElementList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
