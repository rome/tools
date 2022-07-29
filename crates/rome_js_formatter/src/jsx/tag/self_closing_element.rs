use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxSelfClosingElement, JsxSelfClosingElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSelfClosingElement;

impl FormatNodeRule<JsxSelfClosingElement> for FormatJsxSelfClosingElement {
    fn fmt_fields(&self, node: &JsxSelfClosingElement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxSelfClosingElementFields {
            l_angle_token,
            name,
            type_arguments,
            attributes,
            slash_token,
            r_angle_token,
        } = node.as_fields();

        write![
            f,
            [
                l_angle_token.format(),
                name.format(),
                type_arguments.format(),
                space(),
                attributes.format(),
                space(),
                slash_token.format(),
                r_angle_token.format()
            ]
        ]
    }
}
