use crate::prelude::*;
use crate::utils::jsx_utils::contains_text;
use crate::FormatNodeFields;
use crate::{soft_block_indent, FormatElement, Formatter};
use rome_formatter::{fill_elements, group_elements, join_elements, soft_line_break, FormatResult};
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

        let formatted_children = if contains_text(&children) {
            fill_elements(children.format())
        } else {
            join_elements(soft_line_break(), children.format())
        };

        Ok(group_elements(formatted![
            formatter,
            [
                opening_element.format()?,
                soft_block_indent(formatted_children),
                closing_element.format()?
            ]
        ]?))
    }
}
