use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsBooleanLiteralExpression;
use rome_js_syntax::JsBooleanLiteralExpressionFields;

impl FormatNode for JsBooleanLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBooleanLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
