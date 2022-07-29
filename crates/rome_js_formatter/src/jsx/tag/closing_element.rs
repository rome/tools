use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::{JsxClosingElement, JsxClosingElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxClosingElement;

impl FormatNodeRule<JsxClosingElement> for FormatJsxClosingElement {
    fn fmt_fields(
        &self,
        node: &JsxClosingElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsxClosingElementFields {
            l_angle_token,
            slash_token,
            name,
            r_angle_token,
        } = node.as_fields();

        write![
            formatter,
            [group(&format_args![
                l_angle_token.format(),
                slash_token.format(),
                name.format(),
                line_suffix_boundary(),
                r_angle_token.format(),
            ])]
        ]
    }
}
