use crate::prelude::*;
use crate::utils::FormatMemberName;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsConstructorClassMember;
use rome_js_syntax::JsConstructorClassMemberFields;

impl FormatNodeFields<JsConstructorClassMember> for FormatNodeRule<JsConstructorClassMember> {
    fn fmt_fields(node: &JsConstructorClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsConstructorClassMemberFields {
            modifiers,
            name,
            parameters,
            body,
        } = node.as_fields();

        write![
            f,
            [
                modifiers.format(),
                space_token(),
                FormatMemberName::from(name?),
                parameters.format(),
                space_token(),
                body.format()
            ]
        ]
    }
}
