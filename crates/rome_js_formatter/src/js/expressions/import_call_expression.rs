use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsImportCallExpression;
use rome_js_syntax::JsImportCallExpressionFields;

impl FormatNodeFields<JsImportCallExpression> for FormatNodeRule<JsImportCallExpression> {
    fn fmt_fields(node: &JsImportCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCallExpressionFields {
            import_token,
            arguments,
        } = node.as_fields();

        write![f, [import_token.format(), arguments.format(),]]
    }
}
