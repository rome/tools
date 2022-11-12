use crate::prelude::*;
use crate::utils::FormatSemicolon;

use rome_formatter::write;
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
            [
                modifiers.format(),
                space(),
                name.format(),
                group(&parameters.format()),
                FormatSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
