use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsGetterObjectMember;
use rome_js_syntax::JsGetterObjectMemberFields;

impl FormatNodeFields<JsGetterObjectMember> for FormatNodeRule<JsGetterObjectMember> {
    fn fmt_fields(node: &JsGetterObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsGetterObjectMemberFields {
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            body,
        } = node.as_fields();

        write![
            f,
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
