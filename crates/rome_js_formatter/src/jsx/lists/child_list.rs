use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use crate::{FormatElement, Formatter, JsFormatter};
use rome_formatter::{empty_element, fill_elements, group_elements, FormatResult};
use rome_js_syntax::JsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        Ok(group_elements(fill_elements(
            soft_line_break(),
            formatter.format_nodes(node.clone())?,
        )))
    }
}
