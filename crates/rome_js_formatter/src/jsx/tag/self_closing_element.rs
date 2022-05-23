use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxSelfClosingElement, JsxSelfClosingElementFields};

impl FormatNodeFields<JsxSelfClosingElement> for FormatNodeRule<JsxSelfClosingElement> {
    fn format_fields(
        node: &JsxSelfClosingElement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsxSelfClosingElementFields {
            l_angle_token,
            name,
            type_arguments,
            attributes,
            slash_token,
            r_angle_token,
        } = node.as_fields();

        Ok(group_elements(formatted![
            formatter,
            [
                l_angle_token.format(),
                name.format(),
                type_arguments.format(),
                space_token(),
                attributes.format(),
                space_token(),
                slash_token.format(),
                r_angle_token.format()
            ]
        ]?))
    }
}
