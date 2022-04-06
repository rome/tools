use crate::utils::array::format_array_node;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsArrayAssignmentPatternElementList;

impl ToFormatElement for JsArrayAssignmentPatternElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_array_node(self, formatter)
    }
}
