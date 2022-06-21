use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureClassMember;

impl FormatNodeRule<TsPropertySignatureClassMember> for FormatTsPropertySignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsPropertySignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsPropertySignatureClassMemberFields {
            modifiers,
            name,
            property_annotation,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    space_token(),
                    name.format(),
                    property_annotation.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
