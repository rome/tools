use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsImportCallExpression;
use rome_js_syntax::JsImportCallExpressionFields;

impl ToFormatElement for JsImportCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
