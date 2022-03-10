use crate::utils::array::format_array_node;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsArrayBindingPatternElementList;

impl ToFormatElement for JsArrayBindingPatternElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_array_node(self, formatter)
    }
}
