use crate::prelude::*;
use crate::utils::array::write_array_node;
use rome_js_syntax::JsArrayAssignmentPatternElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayAssignmentPatternElementList;

impl FormatRule<JsArrayAssignmentPatternElementList> for FormatJsArrayAssignmentPatternElementList {
    type Context = JsFormatContext;

    fn fmt(
        node: &JsArrayAssignmentPatternElementList,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        write_array_node(node, formatter)
    }
}
