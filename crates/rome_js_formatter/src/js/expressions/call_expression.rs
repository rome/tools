use crate::prelude::*;
use crate::utils::format_call_expression;

use crate::FormatNodeFields;
use rome_js_syntax::JsCallExpression;
use rome_rowan::AstNode;

impl FormatNodeFields<JsCallExpression> for FormatNodeRule<JsCallExpression> {
    fn format_fields(
        node: &JsCallExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_call_expression(node.syntax(), formatter)
    }
}
