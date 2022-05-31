use crate::prelude::*;
use crate::utils::{format_conditional, Conditional};

use crate::FormatNodeFields;
use rome_js_syntax::JsConditionalExpression;

impl FormatNodeFields<JsConditionalExpression> for FormatNodeRule<JsConditionalExpression> {
    fn format_fields(
        node: &JsConditionalExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_conditional(Conditional::Expression(node.clone()), formatter, false)
    }
}
