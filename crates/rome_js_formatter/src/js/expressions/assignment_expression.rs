use crate::prelude::*;
use crate::utils::{format_assignment_like, JsAnyAssignmentLike};
use crate::FormatNodeFields;
use rome_js_syntax::JsAssignmentExpression;

impl FormatNodeFields<JsAssignmentExpression> for FormatNodeRule<JsAssignmentExpression> {
    fn fmt_fields(node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        format_assignment_like(JsAnyAssignmentLike::JsAssignmentExpression(node.clone()), f)
    }
}
