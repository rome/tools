use crate::generated::FormatJsArrayAssignmentPatternElementList;
use crate::prelude::*;
use crate::utils::array::format_array_node;
use rome_js_syntax::JsArrayAssignmentPatternElementList;

impl FormatRule<JsArrayAssignmentPatternElementList> for FormatJsArrayAssignmentPatternElementList {
    type Context = JsFormatContext;

    fn format(
        node: &JsArrayAssignmentPatternElementList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_array_node(node, formatter)
    }
}
