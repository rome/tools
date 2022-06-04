use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsIndexSignatureClassMember;
use rome_js_syntax::TsIndexSignatureClassMemberFields;

impl FormatNodeFields<TsIndexSignatureClassMember> for FormatNodeRule<TsIndexSignatureClassMember> {
    fn fmt_fields(node: &TsIndexSignatureClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexSignatureClassMemberFields {
            modifiers,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    space_token(),
                    l_brack_token.format(),
                    parameter.format(),
                    r_brack_token.format(),
                    type_annotation.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
