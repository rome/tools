use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsShorthandPropertyObjectMember;
use rome_js_syntax::JsShorthandPropertyObjectMemberFields;

impl FormatNodeFields<JsShorthandPropertyObjectMember>
    for FormatNodeRule<JsShorthandPropertyObjectMember>
{
    fn fmt_fields(node: &JsShorthandPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsShorthandPropertyObjectMemberFields { name } = node.as_fields();

        write![f, [name.format()]]
    }
}
