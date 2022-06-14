use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsIndexSignatureTypeMember, TsIndexSignatureTypeMemberFields};

impl FormatNodeFields<TsIndexSignatureTypeMember> for FormatNodeRule<TsIndexSignatureTypeMember> {
    fn fmt_fields(node: &TsIndexSignatureTypeMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexSignatureTypeMemberFields {
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        if let Some(readonly_token) = readonly_token {
            write!(f, [readonly_token.format(), space_token()])?;
        }

        write![
            f,
            [
                l_brack_token.format(),
                parameter.format(),
                r_brack_token.format(),
                type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref()),
            ]
        ]
    }
}
