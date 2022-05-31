use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{write};
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn format_fields(node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write![
            f,
            [
                name.format(),
                colon_token.format(),
                space_token(),
                value.format()
            ]
        ]
    }
}
