use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxOpeningElement, JsxOpeningElementFields};

impl FormatNodeFields<JsxOpeningElement> for FormatNodeRule<JsxOpeningElement> {
    fn fmt_fields(node: &JsxOpeningElement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxOpeningElementFields {
            l_angle_token,
            name,
            type_arguments,
            attributes,
            r_angle_token,
        } = node.as_fields();

        if attributes.is_empty() && type_arguments.is_none() {
            write![
                f,
                [
                    l_angle_token.format(),
                    name.format(),
                    type_arguments.format(),
                    line_suffix_boundary(),
                    attributes.format(),
                    line_suffix_boundary(),
                    r_angle_token.format()
                ]
            ]
        } else {
            write![
                f,
                [
                    l_angle_token.format(),
                    name.format(),
                    type_arguments.format(),
                    line_suffix_boundary(),
                    space_token(),
                    attributes.format(),
                    line_suffix_boundary(),
                    r_angle_token.format()
                ]
            ]
        }
    }
}
