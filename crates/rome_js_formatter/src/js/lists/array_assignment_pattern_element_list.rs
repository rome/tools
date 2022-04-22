use crate::utils::array::format_array_node;
use crate::{Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsArrayAssignmentPatternElementList;

impl Format for JsArrayAssignmentPatternElementList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_array_node(self, formatter)
    }
}
