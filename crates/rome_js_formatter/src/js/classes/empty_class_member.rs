use crate::prelude::*;

use crate::builders::format_removed;
use crate::FormatNodeFields;
use rome_js_syntax::JsEmptyClassMember;
use rome_js_syntax::JsEmptyClassMemberFields;

impl FormatNodeFields<JsEmptyClassMember> for FormatNodeRule<JsEmptyClassMember> {
    fn fmt_fields(node: &JsEmptyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsEmptyClassMemberFields { semicolon_token } = node.as_fields();

        format_removed(&semicolon_token?).fmt(f)
    }
}
