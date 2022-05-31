use crate::prelude::*;
use crate::utils::format_type_member_separator;
use crate::FormatNodeFields;
use rome_js_syntax::{TsSetterSignatureTypeMember, TsSetterSignatureTypeMemberFields};

impl FormatNodeFields<TsSetterSignatureTypeMember> for FormatNodeRule<TsSetterSignatureTypeMember> {
    fn format_fields(
        node: &TsSetterSignatureTypeMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsSetterSignatureTypeMemberFields {
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            separator_token,
        } = node.as_fields();

        let set = set_token.format();
        let name = name.format();
        let l_paren = l_paren_token.format();
        let parameter = parameter.format();
        let r_paren = r_paren_token.format();
        let separator = format_type_member_separator(separator_token, formatter);

        formatted![
            formatter,
            [
                set,
                space_token(),
                name,
                l_paren,
                parameter,
                r_paren,
                separator
            ]
        ]
    }
}
