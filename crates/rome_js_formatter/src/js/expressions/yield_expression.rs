use crate::prelude::*;

use rome_js_syntax::JsYieldExpression;
use rome_js_syntax::JsYieldExpressionFields;

impl FormatNode for JsYieldExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = self.as_fields();

        formatted![formatter, yield_token.format(formatter)?, argument]
    }
}
