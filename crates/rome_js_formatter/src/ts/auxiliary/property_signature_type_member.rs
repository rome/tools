use crate::prelude::*;
use crate::utils::{FormatMemberName, FormatTypeMemberSeparator};

use rome_formatter::write;
use rome_js_syntax::{TsPropertySignatureTypeMember, TsPropertySignatureTypeMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureTypeMember;

impl FormatNodeRule<TsPropertySignatureTypeMember> for FormatTsPropertySignatureTypeMember {
    fn fmt_fields(node: &TsPropertySignatureTypeMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsPropertySignatureTypeMemberFields {
            readonly_token,
            name,
            optional_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                readonly_token.format(),
                space_token(),
                FormatMemberName::from(name?),
                optional_token.format(),
                type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        ]
    }
}
