use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsGetterObjectMember;
use rome_js_syntax::JsGetterObjectMemberFields;

impl FormatNodeFields<JsGetterObjectMember> for FormatNodeRule<JsGetterObjectMember> {
    fn format_fields(
        node: &JsGetterObjectMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsGetterObjectMemberFields {
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            body,
        } = node.as_fields();

        formatted![
            formatter,
            [
                get_token.format(),
                space_token(),
                name.format(),
                l_paren_token.format(),
                r_paren_token.format(),
                return_type.format(),
                space_token(),
                body.format()
            ]
        ]
    }
}
