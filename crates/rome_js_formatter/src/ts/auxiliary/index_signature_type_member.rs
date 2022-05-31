use crate::prelude::*;
use crate::utils::format_type_member_separator;
use crate::FormatNodeFields;
use rome_js_syntax::{TsIndexSignatureTypeMember, TsIndexSignatureTypeMemberFields};

impl FormatNodeFields<TsIndexSignatureTypeMember> for FormatNodeRule<TsIndexSignatureTypeMember> {
    fn format_fields(
        node: &TsIndexSignatureTypeMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsIndexSignatureTypeMemberFields {
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                readonly_token
                    .format()
                    .with_or_empty(|readonly_token| formatted![
                        formatter,
                        [readonly_token, space_token()]
                    ]),
                l_brack_token.format(),
                parameter.format(),
                r_brack_token.format(),
                type_annotation.format(),
                format_type_member_separator(separator_token, formatter),
            ]
        ]
    }
}
