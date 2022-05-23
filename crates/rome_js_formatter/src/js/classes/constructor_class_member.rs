use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsConstructorClassMember;
use rome_js_syntax::JsConstructorClassMemberFields;

impl FormatNodeFields<JsConstructorClassMember> for FormatNodeRule<JsConstructorClassMember> {
    fn format_fields(
        node: &JsConstructorClassMember,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsConstructorClassMemberFields {
            modifiers,
            name,
            parameters,
            body,
        } = node.as_fields();

        formatted![
            formatter,
            [
                modifiers.format(),
                space_token(),
                name.format(),
                parameters.format(),
                space_token(),
                body.format()
            ]
        ]
    }
}
