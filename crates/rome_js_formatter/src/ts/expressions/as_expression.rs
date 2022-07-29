use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsAsExpression;
use rome_js_syntax::TsAsExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsExpression;

impl FormatNodeRule<TsAsExpression> for FormatTsAsExpression {
    fn fmt_fields(&self, node: &TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAsExpressionFields {
            ty,
            as_token,
            expression,
        } = node.as_fields();

        write![
            f,
            [
                expression.format(),
                space(),
                as_token.format(),
                space(),
                ty.format(),
            ]
        ]
    }
}
