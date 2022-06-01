use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsSetterClassMember;
use rome_js_syntax::JsSetterClassMemberFields;

impl FormatNodeFields<JsSetterClassMember> for FormatNodeRule<JsSetterClassMember> {
    fn format_fields(
        node: &JsSetterClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsSetterClassMemberFields {
            modifiers,
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
                modifiers.format(),
                space_token(),
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
