use crate::prelude::*;
use crate::utils::format_type_member_separator;
use crate::FormatNodeFields;
use rome_js_syntax::{TsMethodSignatureTypeMember, TsMethodSignatureTypeMemberFields};

impl FormatNodeFields<TsMethodSignatureTypeMember> for FormatNodeRule<TsMethodSignatureTypeMember> {
    fn format_fields(
        node: &TsMethodSignatureTypeMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsMethodSignatureTypeMemberFields {
            name,
            optional_token,
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = node.as_fields();

        let separator = format_type_member_separator(separator_token, formatter);
        formatted![
            formatter,
            [
                name.format(),
                optional_token.format(),
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                separator
            ]
        ]
    }
}
