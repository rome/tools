use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsConstructorSignatureClassMember;
use rome_js_syntax::TsConstructorSignatureClassMemberFields;

impl FormatNodeFields<TsConstructorSignatureClassMember>
    for FormatNodeRule<TsConstructorSignatureClassMember>
{
    fn fmt_fields(
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
                    space_token(),
                    name.format(),
                    parameters.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
