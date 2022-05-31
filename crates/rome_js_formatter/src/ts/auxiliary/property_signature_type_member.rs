use crate::prelude::*;
use crate::utils::format_type_member_separator;
use crate::FormatNodeFields;
use rome_js_syntax::{TsPropertySignatureTypeMember, TsPropertySignatureTypeMemberFields};

impl FormatNodeFields<TsPropertySignatureTypeMember>
    for FormatNodeRule<TsPropertySignatureTypeMember>
{
    fn format_fields(
        node: &TsPropertySignatureTypeMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsPropertySignatureTypeMemberFields {
            readonly_token,
            name,
            optional_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        let separator = format_type_member_separator(separator_token, formatter);

        formatted![
            formatter,
            [
                readonly_token.format(),
                space_token(),
                name.format(),
                optional_token.format(),
                type_annotation.format(),
                separator
            ]
        ]
    }
}
