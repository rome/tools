use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsComputedMemberName;
use rome_js_syntax::JsComputedMemberNameFields;

impl FormatNodeFields<JsComputedMemberName> for FormatNodeRule<JsComputedMemberName> {
    fn fmt_fields(node: &JsComputedMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsComputedMemberNameFields {
            l_brack_token,
            expression,
            r_brack_token,
        } = node.as_fields();

        write![
            f,
            [
                l_brack_token.format(),
                expression.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
