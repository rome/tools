use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsImportCallExpression;
use rome_js_syntax::JsImportCallExpressionFields;

impl FormatNodeFields<JsImportCallExpression> for FormatNodeRule<JsImportCallExpression> {
    fn format_fields(
        node: &JsImportCallExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportCallExpressionFields {
            import_token,
            arguments,
        } = node.as_fields();

        formatted![formatter, [import_token.format(), arguments.format(),]]
    }
}
