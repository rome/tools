use crate::prelude::*;
use crate::FormatNodeFields;
use crate::{FormatElement, Formatter};
use rome_formatter::{group_elements, FormatResult};
use rome_js_syntax::{JsxExpressionChild, JsxExpressionChildFields};

impl FormatNodeFields<JsxExpressionChild> for FormatNodeRule<JsxExpressionChild> {
    fn format_fields(
        node: &JsxExpressionChild,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsxExpressionChildFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        Ok(group_elements(formatted![
            formatter,
            [
                l_curly_token.format(),
                expression.format(),
                line_suffix_boundary(),
                r_curly_token.format()
            ]
        ]?))
    }
}
