use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsIdentifierExpression;
use rome_js_syntax::JsIdentifierExpressionFields;

impl FormatNodeFields<JsIdentifierExpression> for FormatNodeRule<JsIdentifierExpression> {
    fn fmt_fields(node: &JsIdentifierExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierExpressionFields { name } = node.as_fields();

        write![f, [name.format()]]
    }
}
