use crate::generated::FormatJsArrayBindingPatternElementList;
use crate::prelude::*;
use crate::utils::array::write_array_node;
use rome_js_syntax::JsArrayBindingPatternElementList;

impl FormatRule<JsArrayBindingPatternElementList> for FormatJsArrayBindingPatternElementList {
    type Context = JsFormatContext;

    fn fmt(
        node: &JsArrayBindingPatternElementList,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        write_array_node(node, formatter)
    }
}
