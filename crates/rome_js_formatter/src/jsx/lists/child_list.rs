use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use rome_formatter::{group_elements, join_elements, soft_line_break, FormatResult};
use rome_js_syntax::JsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsxChildList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(group_elements(fill_elements(
            soft_line_break(),
            formatter.format_nodes(node.clone())?,
        )))
    }
}
