use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsImportCallExpression;
use rome_js_syntax::JsImportCallExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportCallExpression;

impl FormatNodeRule<JsImportCallExpression> for FormatJsImportCallExpression {
    fn fmt_fields(&self, node: &JsImportCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCallExpressionFields {
            import_token,
            arguments,
        } = node.as_fields();

        write![f, [import_token.format(), arguments.format(),]]
    }
}
