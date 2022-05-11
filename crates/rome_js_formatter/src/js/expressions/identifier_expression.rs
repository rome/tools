use crate::prelude::*;

use rome_js_syntax::JsIdentifierExpression;
use rome_js_syntax::JsIdentifierExpressionFields;

impl FormatNode for JsIdentifierExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierExpressionFields { name } = self.as_fields();

        name.format(formatter)
    }
}
