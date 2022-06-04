use crate::prelude::*;
use crate::utils::{format_conditional, Conditional};

use crate::FormatNodeFields;
use rome_js_syntax::JsConditionalExpression;

impl FormatNodeFields<JsConditionalExpression> for FormatNodeRule<JsConditionalExpression> {
    fn fmt_fields(node: &JsConditionalExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_conditional(&Conditional::Expression(node.clone()), formatter, false)
    }
}
