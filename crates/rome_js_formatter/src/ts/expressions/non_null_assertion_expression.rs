use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsNonNullAssertionExpression;
use rome_js_syntax::TsNonNullAssertionExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonNullAssertionExpression;

impl FormatNodeRule<TsNonNullAssertionExpression> for FormatTsNonNullAssertionExpression {
    fn fmt_fields(
        &self,
        node: &TsNonNullAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsNonNullAssertionExpressionFields {
            expression,
            excl_token,
        } = node.as_fields();

        write![f, [expression.format(), excl_token.format()]]
    }
}
