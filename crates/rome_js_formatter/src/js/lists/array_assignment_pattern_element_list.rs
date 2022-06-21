use crate::prelude::*;
use crate::utils::array::write_array_node;
use rome_js_syntax::JsArrayAssignmentPatternElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayAssignmentPatternElementList;

impl FormatNodeRule<JsArrayAssignmentPatternElementList>
    for FormatJsArrayAssignmentPatternElementList
{
    fn fmt_fields(
        &self,
        node: &JsArrayAssignmentPatternElementList,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        write_array_node(node, formatter)
    }
}
