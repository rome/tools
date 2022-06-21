use crate::prelude::*;
use crate::utils::array::write_array_node;
use rome_js_syntax::JsArrayBindingPatternElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayBindingPatternElementList;

impl FormatNodeRule<JsArrayBindingPatternElementList> for FormatJsArrayBindingPatternElementList {
    fn fmt_fields(
        &self,
        node: &JsArrayBindingPatternElementList,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        write_array_node(node, formatter)
    }
}
