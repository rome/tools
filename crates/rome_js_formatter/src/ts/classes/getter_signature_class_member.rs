use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::{TsGetterSignatureClassMember, TsGetterSignatureClassMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsGetterSignatureClassMember;

impl FormatNodeRule<TsGetterSignatureClassMember> for FormatTsGetterSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsGetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsGetterSignatureClassMemberFields {
            modifiers,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    space(),
                    get_token.format(),
                    space(),
                    name.format(),
                    l_paren_token.format(),
                    r_paren_token.format(),
                    return_type.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
