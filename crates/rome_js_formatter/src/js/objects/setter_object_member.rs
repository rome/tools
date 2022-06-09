use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsSetterObjectMember;
use rome_js_syntax::JsSetterObjectMemberFields;

impl FormatNodeFields<JsSetterObjectMember> for FormatNodeRule<JsSetterObjectMember> {
    fn fmt_fields(node: &JsSetterObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSetterObjectMemberFields {
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            body,
        } = node.as_fields();

        write![
            f,
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
