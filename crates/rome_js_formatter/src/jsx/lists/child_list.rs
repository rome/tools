use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use rome_formatter::{group_elements, join_elements, soft_line_break, FormatResult};
use rome_js_syntax::JsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        Ok(group_elements(join_elements(
            soft_line_break(),
            formatter.format_nodes(node.clone())?,
        )))
    }
}
