use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxClosingElement, JsxClosingElementFields};

impl FormatNodeFields<JsxClosingElement> for FormatNodeRule<JsxClosingElement> {
    fn format_fields(
        node: &JsxClosingElement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsxClosingElementFields {
            l_angle_token,
            slash_token,
            name,
            r_angle_token,
        } = node.as_fields();

        Ok(group_elements(formatted![
            formatter,
            [
                l_angle_token.format(),
                slash_token.format(),
                name.format(),
                line_suffix_boundary(),
                r_angle_token.format(),
            ]
        ]?))
    }
}
