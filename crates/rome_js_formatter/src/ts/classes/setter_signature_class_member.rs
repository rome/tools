use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{TsSetterSignatureClassMember, TsSetterSignatureClassMemberFields};

impl FormatNodeFields<TsSetterSignatureClassMember>
    for FormatNodeRule<TsSetterSignatureClassMember>
{
    fn format_fields(
        node: &TsSetterSignatureClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsSetterSignatureClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        let set_token = set_token.format();
        let name = name.format();
        let l_paren_token = l_paren_token.format();
        let parameters = parameter.format();
        let r_paren_token = r_paren_token.format();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    modifiers.format(),
                    space_token(),
                    set_token,
                    space_token(),
                    name,
                    l_paren_token,
                    parameters,
                    r_paren_token,
                ]
            ]?,
            semicolon_token,
        )
    }
}
