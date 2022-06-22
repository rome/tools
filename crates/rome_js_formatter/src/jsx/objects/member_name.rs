use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxMemberName, JsxMemberNameFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxMemberName;

impl FormatNodeRule<JsxMemberName> for FormatJsxMemberName {
    fn fmt_fields(&self, node: &JsxMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxMemberNameFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        write![f, [object.format(), dot_token.format(), member.format(),]]
    }
}
