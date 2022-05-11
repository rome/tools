use crate::prelude::*;

use rome_js_syntax::JsThisExpression;
use rome_js_syntax::JsThisExpressionFields;

impl FormatNode for JsThisExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsThisExpressionFields { this_token } = self.as_fields();

        this_token.format(formatter)
    }
}
