use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsComputedMemberName;
use rome_js_syntax::JsComputedMemberNameFields;

impl FormatNodeFields<JsComputedMemberName> for FormatNodeRule<JsComputedMemberName> {
    fn format_fields(
        node: &JsComputedMemberName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsComputedMemberNameFields {
            l_brack_token,
            expression,
            r_brack_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                l_brack_token.format(),
                expression.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
