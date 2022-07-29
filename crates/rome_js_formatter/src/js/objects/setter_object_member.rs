use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsSetterObjectMember;
use rome_js_syntax::JsSetterObjectMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSetterObjectMember;

impl FormatNodeRule<JsSetterObjectMember> for FormatJsSetterObjectMember {
    fn fmt_fields(&self, node: &JsSetterObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
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
                space(),
                name.format(),
                l_paren_token.format(),
                parameter.format(),
                r_paren_token.format(),
                space(),
                body.format(),
            ]
        ]
    }
}
