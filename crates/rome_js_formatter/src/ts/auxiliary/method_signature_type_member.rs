use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use rome_formatter::write;
use rome_js_syntax::{TsMethodSignatureTypeMember, TsMethodSignatureTypeMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureTypeMember;

impl FormatNodeRule<TsMethodSignatureTypeMember> for FormatTsMethodSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsMethodSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsMethodSignatureTypeMemberFields {
            name,
            optional_token,
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                name.format(),
                optional_token.format(),
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        ]
    }
}
