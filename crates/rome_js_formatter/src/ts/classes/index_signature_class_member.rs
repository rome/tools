use crate::prelude::*;
use crate::utils::{FormatOptionalSemicolon};


use rome_formatter::write;
use rome_js_syntax::TsIndexSignatureClassMemberFields;
use rome_js_syntax::{TsIndexSignatureClassMember};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexSignatureClassMember;

impl FormatNodeRule<TsIndexSignatureClassMember> for FormatTsIndexSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsIndexSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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
            [
                modifiers.format(),
                space(),
                l_brack_token.format(),
                parameter.format(),
                r_brack_token.format(),
                type_annotation.format(),
                FormatOptionalSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
