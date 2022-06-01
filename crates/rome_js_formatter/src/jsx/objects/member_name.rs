use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxMemberName, JsxMemberNameFields};

impl FormatNodeFields<JsxMemberName> for FormatNodeRule<JsxMemberName> {
    fn format_fields(node: &JsxMemberName, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsxMemberNameFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        formatted![
            formatter,
            [object.format(), dot_token.format(), member.format(),]
        ]
    }
}
