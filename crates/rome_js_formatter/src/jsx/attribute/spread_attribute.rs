use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxSpreadAttribute, JsxSpreadAttributeFields};

impl FormatNodeFields<JsxSpreadAttribute> for FormatNodeRule<JsxSpreadAttribute> {
    fn format_fields(
        node: &JsxSpreadAttribute,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxSpreadAttributeFields {
            l_curly_token,
            dotdotdot_token,
            argument,
            r_curly_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                l_curly_token.format(),
                dotdotdot_token.format(),
                argument.format(),
                r_curly_token.format(),
            ]
        ]
    }
}
