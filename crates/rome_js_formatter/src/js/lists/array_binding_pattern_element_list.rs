use crate::generated::FormatJsArrayBindingPatternElementList;
use crate::prelude::*;
use crate::utils::array::format_array_node;
use rome_js_syntax::JsArrayBindingPatternElementList;

impl FormatRule<JsArrayBindingPatternElementList> for FormatJsArrayBindingPatternElementList {
    type Context = JsFormatContext;

    fn format(
        node: &JsArrayBindingPatternElementList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_array_node(node, formatter)
    }
}
