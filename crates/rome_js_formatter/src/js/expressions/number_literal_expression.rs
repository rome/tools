use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsNumberLiteralExpressionFields;
use rome_js_syntax::{JsNumberLiteralExpression, JsStaticMemberExpression};

impl FormatNodeFields<JsNumberLiteralExpression> for FormatNodeRule<JsNumberLiteralExpression> {
    fn fmt_fields(node: &JsNumberLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNumberLiteralExpressionFields { value_token } = node.as_fields();
        let value_token = value_token?;

        if let Some(static_member_expression) = node.parent::<JsStaticMemberExpression>() {
            if static_member_expression.object()?.syntax() == node.syntax() {
                return write!(
                    f,
                    [format_parenthesize(
                        Some(value_token.clone()),
                        &value_token.format(),
                        Some(value_token.clone())
                    )]
                );
            }
        }

        write![f, [value_token.format()]]
    }
}
