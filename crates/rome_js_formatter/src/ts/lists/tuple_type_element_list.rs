use crate::prelude::*;
use rome_js_syntax::{JsSyntaxKind, TsTupleTypeElementList};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTupleTypeElementList;

impl FormatNodeRule<TsTupleTypeElementList> for FormatTsTupleTypeElementList {
    fn fmt_fields(&self, node: &TsTupleTypeElementList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
