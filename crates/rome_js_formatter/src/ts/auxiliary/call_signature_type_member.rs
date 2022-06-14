use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsCallSignatureTypeMember, TsCallSignatureTypeMemberFields};

impl FormatNodeFields<TsCallSignatureTypeMember> for FormatNodeRule<TsCallSignatureTypeMember> {
    fn fmt_fields(node: &TsCallSignatureTypeMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsCallSignatureTypeMemberFields {
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = node.as_fields();

        write!(
            f,
            [
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        )
    }
}
