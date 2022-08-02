use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::TsConstructorSignatureClassMember;
use rome_js_syntax::TsConstructorSignatureClassMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsConstructorSignatureClassMember;

impl FormatNodeRule<TsConstructorSignatureClassMember> for FormatTsConstructorSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsConstructorSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsConstructorSignatureClassMemberFields {
            modifiers,
            name,
            parameters,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    space(),
                    name.format(),
                    group(&parameters.format()),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
