use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use crate::FormatNodeFields;
use rome_js_syntax::JsInstanceofExpression;

impl FormatNodeFields<JsInstanceofExpression> for FormatNodeRule<JsInstanceofExpression> {
    fn fmt_fields(node: &JsInstanceofExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInstanceofExpression(node.clone()),
            formatter,
        )
    }
}
