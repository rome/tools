use crate::prelude::*;
use crate::utils::JsAnyConditional;

use rome_js_syntax::JsConditionalExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsConditionalExpression;

impl FormatNodeRule<JsConditionalExpression> for FormatJsConditionalExpression {
    fn fmt_fields(
        &self,
        node: &JsConditionalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        JsAnyConditional::from(node.clone()).fmt(formatter)
    }
}
