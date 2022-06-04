use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsSetterSignatureClassMember, TsSetterSignatureClassMemberFields};

impl FormatNodeFields<TsSetterSignatureClassMember>
    for FormatNodeRule<TsSetterSignatureClassMember>
{
    fn fmt_fields(node: &TsSetterSignatureClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsSetterSignatureClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        let setter = format_with(|f| {
            write!(
                f,
                [
                    modifiers.format(),
                    space_token(),
                    set_token.format(),
                    space_token(),
                    name.format(),
                    l_paren_token.format(),
                    parameter.format(),
                    r_paren_token.format(),
                ]
            )
        });

        write!(
            f,
            [FormatWithSemicolon::new(&setter, semicolon_token.as_ref())]
        )
    }
}
