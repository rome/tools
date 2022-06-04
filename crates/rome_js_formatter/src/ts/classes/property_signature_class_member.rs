use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

impl FormatNodeFields<TsPropertySignatureClassMember>
    for FormatNodeRule<TsPropertySignatureClassMember>
{
    fn fmt_fields(node: &TsPropertySignatureClassMember, f: &mut JsFormatter) -> FormatResult<()> {
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
