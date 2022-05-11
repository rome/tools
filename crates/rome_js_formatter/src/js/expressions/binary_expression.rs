use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use rome_js_syntax::JsBinaryExpression;

impl FormatNode for JsBinaryExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsBinaryExpression(self.clone()),
            formatter,
        )
    }
}
