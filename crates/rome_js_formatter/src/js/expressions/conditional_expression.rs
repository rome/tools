use crate::prelude::*;
use crate::utils::{format_conditional, Conditional};

use rome_js_syntax::JsConditionalExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsConditionalExpression;

impl FormatNodeRule<JsConditionalExpression> for FormatJsConditionalExpression {
    fn fmt_fields(
        &self,
        node: &JsConditionalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_conditional(&Conditional::Expression(node.clone()), formatter, false)
    }
}
