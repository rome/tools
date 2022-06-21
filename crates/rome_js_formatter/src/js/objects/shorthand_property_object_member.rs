use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsShorthandPropertyObjectMember;
use rome_js_syntax::JsShorthandPropertyObjectMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsShorthandPropertyObjectMember;

impl FormatNodeRule<JsShorthandPropertyObjectMember> for FormatJsShorthandPropertyObjectMember {
    fn fmt_fields(node: &JsShorthandPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsShorthandPropertyObjectMemberFields { name } = node.as_fields();

        write![f, [name.format()]]
    }
}
