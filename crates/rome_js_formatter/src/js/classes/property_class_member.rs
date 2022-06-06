use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::{FormatMemberName, FormatWithSemicolon};
use crate::FormatNodeFields;
use rome_js_syntax::JsPropertyClassMember;
use rome_js_syntax::JsPropertyClassMemberFields;

impl FormatNodeFields<JsPropertyClassMember> for FormatNodeRule<JsPropertyClassMember> {
    fn fmt_fields(node: &JsPropertyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPropertyClassMemberFields {
            modifiers,
            name,
            property_annotation,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    space_token(),
                    FormatMemberName::from(name?),
                    property_annotation.format(),
                    value
                        .format()
                        .with_or_empty(|node, f| write![f, [space_token(), node]]),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
