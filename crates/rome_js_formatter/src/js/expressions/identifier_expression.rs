use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsIdentifierExpression;
use rome_js_syntax::JsIdentifierExpressionFields;

impl FormatNodeFields<JsIdentifierExpression> for FormatNodeRule<JsIdentifierExpression> {
    fn format_fields(
        node: &JsIdentifierExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsIdentifierExpressionFields { name } = node.as_fields();

        formatted![formatter, [name.format()]]
    }
}
