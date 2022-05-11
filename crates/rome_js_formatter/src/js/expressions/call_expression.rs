use crate::prelude::*;
use crate::utils::format_call_expression;

use rome_js_syntax::JsCallExpression;
use rome_rowan::AstNode;

impl FormatNode for JsCallExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_call_expression(self.syntax(), formatter)
    }
}
