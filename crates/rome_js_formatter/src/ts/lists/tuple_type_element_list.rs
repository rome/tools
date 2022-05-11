use crate::formatter::TrailingSeparator;
use crate::generated::FormatTsTupleTypeElementList;
use crate::prelude::*;
use rome_js_syntax::TsTupleTypeElementList;

impl FormatRule<TsTupleTypeElementList> for FormatTsTupleTypeElementList {
    type Options = JsFormatOptions;

    fn format(
        node: &TsTupleTypeElementList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
