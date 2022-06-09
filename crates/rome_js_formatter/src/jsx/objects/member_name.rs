use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxMemberName, JsxMemberNameFields};

impl FormatNodeFields<JsxMemberName> for FormatNodeRule<JsxMemberName> {
    fn fmt_fields(node: &JsxMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxMemberNameFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        write![f, [object.format(), dot_token.format(), member.format(),]]
    }
}
