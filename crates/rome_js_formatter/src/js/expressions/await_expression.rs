use crate::prelude::*;

use rome_js_syntax::JsAwaitExpression;
use rome_js_syntax::JsAwaitExpressionFields;

impl FormatNode for JsAwaitExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = self.as_fields();

        formatted![
            formatter,
            await_token.format(formatter)?,
            space_token(),
            argument.format(formatter)?,
        ]
    }
}
