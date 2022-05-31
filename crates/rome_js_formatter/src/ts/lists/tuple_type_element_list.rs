use crate::generated::FormatTsTupleTypeElementList;
use crate::prelude::*;
use rome_js_syntax::TsTupleTypeElementList;

impl FormatRule<TsTupleTypeElementList> for FormatTsTupleTypeElementList {
    type Context = JsFormatContext;

    fn format(
        node: &TsTupleTypeElementList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
