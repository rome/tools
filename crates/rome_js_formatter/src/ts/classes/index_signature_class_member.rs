use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::TsIndexSignatureClassMember;
use rome_js_syntax::TsIndexSignatureClassMemberFields;

impl FormatNodeFields<TsIndexSignatureClassMember> for FormatNodeRule<TsIndexSignatureClassMember> {
    fn format_fields(
        node: &TsIndexSignatureClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsIndexSignatureClassMemberFields {
            modifiers,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    modifiers.format(),
                    space_token(),
                    l_brack_token.format(),
                    parameter.format(),
                    r_brack_token.format(),
                    type_annotation.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
