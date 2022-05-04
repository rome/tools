use crate::prelude::*;
use crate::FormatNodeFields;
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

        formatted![
            formatter,
            [
                opening_element.format(),
                soft_block_indent(formatted![formatter, [children.format()]]?),
                closing_element.format()?
            ]
        ]
    }
}
