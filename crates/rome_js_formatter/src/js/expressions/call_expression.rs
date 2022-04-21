use crate::utils::format_call_expression;
use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsCallExpression;
use rome_rowan::AstNode;

impl FormatNode for JsCallExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_call_expression(self.syntax(), formatter)
    }
}
