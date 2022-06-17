use crate::prelude::*;

use crate::utils::JsAnyAssignmentLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsPropertyObjectMember;

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn fmt_fields(node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
