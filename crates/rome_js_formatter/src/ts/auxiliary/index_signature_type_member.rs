use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsIndexSignatureTypeMember, TsIndexSignatureTypeMemberFields};

impl FormatNodeFields<TsIndexSignatureTypeMember> for FormatNodeRule<TsIndexSignatureTypeMember> {
    fn format_fields(node: &TsIndexSignatureTypeMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexSignatureTypeMemberFields {
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                readonly_token
                    .format()
                    .with_or_empty(|readonly_token, f| write![f, [readonly_token, space_token()]]),
                l_brack_token.format(),
                parameter.format(),
                r_brack_token.format(),
                type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref()),
            ]
        ]
    }
}
