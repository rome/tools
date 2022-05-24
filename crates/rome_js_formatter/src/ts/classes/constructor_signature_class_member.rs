use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::TsConstructorSignatureClassMember;
use rome_js_syntax::TsConstructorSignatureClassMemberFields;

impl FormatNodeFields<TsConstructorSignatureClassMember>
    for FormatNodeRule<TsConstructorSignatureClassMember>
{
    fn format_fields(
        node: &TsConstructorSignatureClassMember,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsConstructorSignatureClassMemberFields {
            modifiers,
            name,
            parameters,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    modifiers.format(),
                    space_token(),
                    name.format(),
                    parameters.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
