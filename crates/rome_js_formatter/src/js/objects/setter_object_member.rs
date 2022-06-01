use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsSetterObjectMember;
use rome_js_syntax::JsSetterObjectMemberFields;

impl FormatNodeFields<JsSetterObjectMember> for FormatNodeRule<JsSetterObjectMember> {
    fn format_fields(
        node: &JsSetterObjectMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsSetterObjectMemberFields {
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            body,
        } = node.as_fields();

        formatted![
            formatter,
            [
                set_token.format(),
                space_token(),
                name.format(),
                l_paren_token.format(),
                parameter.format(),
                r_paren_token.format(),
                space_token(),
                body.format(),
            ]
        ]
    }
}
