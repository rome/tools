use crate::prelude::*;

use crate::utils::JsAnyAssignmentLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsAssignmentExpression;

impl FormatNodeFields<JsAssignmentExpression> for FormatNodeRule<JsAssignmentExpression> {
    fn fmt_fields(node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
