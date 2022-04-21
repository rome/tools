use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsImportCallExpression;
use rome_js_syntax::JsImportCallExpressionFields;

impl FormatNode for JsImportCallExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportCallExpressionFields {
            import_token,
            arguments,
        } = self.as_fields();

        Ok(format_elements![
            import_token.format(formatter)?,
            arguments.format(formatter)?,
        ])
    }
}
