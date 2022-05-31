use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

impl FormatNodeFields<TsPropertySignatureClassMember>
    for FormatNodeRule<TsPropertySignatureClassMember>
{
    fn format_fields(
        node: &TsPropertySignatureClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsPropertySignatureClassMemberFields {
            modifiers,
            name,
            property_annotation,
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
                    property_annotation.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
