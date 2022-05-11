use crate::prelude::*;

use rome_js_syntax::JsImportCallExpression;
use rome_js_syntax::JsImportCallExpressionFields;

impl FormatNode for JsImportCallExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportCallExpressionFields {
            import_token,
            arguments,
        } = self.as_fields();

        formatted![
            formatter,
            import_token.format(formatter)?,
            arguments.format(formatter)?,
        ]
    }
}
