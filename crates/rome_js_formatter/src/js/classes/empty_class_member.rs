use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsEmptyClassMember;
use rome_js_syntax::JsEmptyClassMemberFields;

impl FormatNodeFields<JsEmptyClassMember> for FormatNodeRule<JsEmptyClassMember> {
    fn fmt_fields(node: &JsEmptyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsEmptyClassMemberFields { semicolon_token } = node.as_fields();

        write!(f, [format_replaced(&semicolon_token?, &empty_element())])
    }
}
