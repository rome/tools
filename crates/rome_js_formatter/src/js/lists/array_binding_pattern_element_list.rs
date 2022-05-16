use crate::generated::FormatJsArrayBindingPatternElementList;
use crate::prelude::*;
use crate::utils::array::format_array_node;
use rome_js_syntax::JsArrayBindingPatternElementList;

impl FormatRule<JsArrayBindingPatternElementList> for FormatJsArrayBindingPatternElementList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsArrayBindingPatternElementList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        format_array_node(node, formatter)
    }
}
