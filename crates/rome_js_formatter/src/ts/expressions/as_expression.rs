use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsAsExpression;
use rome_js_syntax::TsAsExpressionFields;

impl FormatNodeFields<TsAsExpression> for FormatNodeRule<TsAsExpression> {
    fn fmt_fields(node: &TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAsExpressionFields {
            ty,
            as_token,
            expression,
        } = node.as_fields();

        write![
            f,
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
