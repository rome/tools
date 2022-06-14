use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use crate::FormatNodeFields;
use rome_js_syntax::JsBinaryExpression;

impl FormatNodeFields<JsBinaryExpression> for FormatNodeRule<JsBinaryExpression> {
    fn fmt_fields(node: &JsBinaryExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsBinaryExpression(node.clone()),
            formatter,
        )
    }
}
