use crate::prelude::*;
use crate::utils::format_type_member_separator;
use crate::FormatNodeFields;
use rome_js_syntax::{TsConstructSignatureTypeMember, TsConstructSignatureTypeMemberFields};

impl FormatNodeFields<TsConstructSignatureTypeMember>
    for FormatNodeRule<TsConstructSignatureTypeMember>
{
    fn format_fields(
        node: &TsConstructSignatureTypeMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsConstructSignatureTypeMemberFields {
            new_token,
            type_parameters,
            parameters,
            type_annotation,
            separator_token,
        } = node.as_fields();

        let separator_token = format_type_member_separator(separator_token, formatter);

        formatted![
            formatter,
            [
                new_token.format(),
                space_token(),
                type_parameters.format(),
                parameters.format(),
                type_annotation.format(),
                separator_token,
            ]
        ]
    }
}
