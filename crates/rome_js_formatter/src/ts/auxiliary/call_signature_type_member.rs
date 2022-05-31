use crate::prelude::*;
use crate::utils::format_type_member_separator;
use crate::FormatNodeFields;
use rome_js_syntax::{TsCallSignatureTypeMember, TsCallSignatureTypeMemberFields};

impl FormatNodeFields<TsCallSignatureTypeMember> for FormatNodeRule<TsCallSignatureTypeMember> {
    fn format_fields(
        node: &TsCallSignatureTypeMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsCallSignatureTypeMemberFields {
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = node.as_fields();

        let separator = format_type_member_separator(separator_token, formatter);

        formatted![
            formatter,
            [
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                separator
            ]
        ]
    }
}
