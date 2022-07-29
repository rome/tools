use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::{TsSetterSignatureClassMember, TsSetterSignatureClassMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSetterSignatureClassMember;

impl FormatNodeRule<TsSetterSignatureClassMember> for FormatTsSetterSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsSetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsSetterSignatureClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    space(),
                    set_token.format(),
                    space(),
                    name.format(),
                    l_paren_token.format(),
                    parameter.format(),
                    r_paren_token.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
