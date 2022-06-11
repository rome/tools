use crate::prelude::*;

use crate::utils::{format_assignment_like, JsAnyAssignmentLike};
use crate::FormatNodeFields;
use rome_js_syntax::JsPropertyObjectMember;

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn fmt_fields(node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        format_assignment_like(JsAnyAssignmentLike::JsPropertyObjectMember(node.clone()), f)
    }
}
