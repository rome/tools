use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{TsGetterSignatureClassMember, TsGetterSignatureClassMemberFields};

impl FormatNodeFields<TsGetterSignatureClassMember>
    for FormatNodeRule<TsGetterSignatureClassMember>
{
    fn format_fields(
        node: &TsGetterSignatureClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsGetterSignatureClassMemberFields {
            modifiers,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
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
                ]
            ]?,
            semicolon_token,
        )
    }
}
