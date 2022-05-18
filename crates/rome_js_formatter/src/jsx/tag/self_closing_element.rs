use crate::prelude::*;
use crate::utils::jsx_utils::should_wrap_element_in_parens;
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

        let element = formatted![
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
