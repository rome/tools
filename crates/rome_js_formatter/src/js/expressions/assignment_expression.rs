use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;

use rome_formatter::write;
use rome_js_syntax::JsAssignmentExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsAssignmentExpression;

impl FormatNodeRule<JsAssignmentExpression> for FormatJsAssignmentExpression {
    fn fmt_fields(&self, node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
