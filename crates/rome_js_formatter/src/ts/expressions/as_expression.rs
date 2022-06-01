use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsAsExpression;
use rome_js_syntax::TsAsExpressionFields;

impl FormatNodeFields<TsAsExpression> for FormatNodeRule<TsAsExpression> {
    fn format_fields(
        node: &TsAsExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsAsExpressionFields {
            ty,
            as_token,
            expression,
        } = node.as_fields();

        formatted![
            formatter,
            [
                expression.format(),
                space_token(),
                as_token.format(),
                space_token(),
                ty.format(),
            ]
        ]
    }
}
