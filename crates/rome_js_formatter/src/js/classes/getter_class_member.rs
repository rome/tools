use crate::prelude::*;
use crate::utils::FormatMemberName;

use rome_formatter::write;
use rome_js_syntax::JsGetterClassMember;
use rome_js_syntax::JsGetterClassMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsGetterClassMember;

impl FormatNodeRule<JsGetterClassMember> for FormatJsGetterClassMember {
    fn fmt_fields(&self, node: &JsGetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsGetterClassMemberFields {
            modifiers,
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
                modifiers.format(),
                space_token(),
                get_token.format(),
                space_token(),
                FormatMemberName::from(name?),
                l_paren_token.format(),
                r_paren_token.format(),
                return_type.format(),
                space_token(),
                body.format()
            ]
        ]
    }
}
