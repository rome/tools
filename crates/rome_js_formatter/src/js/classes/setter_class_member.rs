use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsSetterClassMember;
use rome_js_syntax::JsSetterClassMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSetterClassMember;

impl FormatNodeRule<JsSetterClassMember> for FormatJsSetterClassMember {
    fn fmt_fields(&self, node: &JsSetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSetterClassMemberFields {
            modifiers,
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
                modifiers.format(),
                space(),
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
