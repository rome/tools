use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsGetterClassMember;
use rome_js_syntax::JsGetterClassMemberFields;

impl FormatNodeFields<JsGetterClassMember> for FormatNodeRule<JsGetterClassMember> {
    fn format_fields(
        node: &JsGetterClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsGetterClassMemberFields {
            modifiers,
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
                modifiers.format(),
                space_token(),
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
