use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsShorthandPropertyObjectMember;
use rome_js_syntax::JsShorthandPropertyObjectMemberFields;

impl FormatNodeFields<JsShorthandPropertyObjectMember>
    for FormatNodeRule<JsShorthandPropertyObjectMember>
{
    fn format_fields(
        node: &JsShorthandPropertyObjectMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsShorthandPropertyObjectMemberFields { name } = node.as_fields();

        formatted![formatter, [name.format()]]
    }
}
