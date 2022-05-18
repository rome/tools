use crate::prelude::*;
use crate::utils::jsx_utils::should_wrap_element_in_parens;
use crate::FormatNodeFields;
use crate::{soft_block_indent, FormatElement, Formatter};
use rome_formatter::{group_elements, FormatResult};
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

        let element = formatted![
            formatter,
            [
                opening_element.format(),
                soft_block_indent(formatted![formatter, [children.format()]]?),
                closing_element.format()
            ]
        ]?;

        if should_wrap_element_in_parens(node.syntax()) {
            Ok(group_elements(formatted![
                formatter,
                [
                    if_group_breaks(token("(")),
                    soft_block_indent(element),
                    if_group_breaks(token(")"))
                ]
            ]?))
        } else {
            Ok(group_elements(element))
        }
    }
}
