use crate::prelude::*;
use crate::utils::jsx_utils::is_jsx_inside_arrow_function_inside_call_inside_expression_child;
use crate::FormatNodeFields;
use crate::{soft_block_indent, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{JsxElement, JsxElementFields};

impl FormatNodeFields<JsxElement> for FormatNodeRule<JsxElement> {
    fn format_fields(
        node: &JsxElement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsxElementFields {
            opening_element,
            children,
            closing_element,
        } = node.as_fields();

        let indent =
            if is_jsx_inside_arrow_function_inside_call_inside_expression_child(node.syntax()) {
                block_indent
            } else {
                soft_block_indent
            };

        Ok(group_elements(formatted![
            formatter,
            [
                opening_element.format(),
                indent(formatted![formatter, [children.format()]]?),
                closing_element.format()
            ]
        ]?))
    }
}
