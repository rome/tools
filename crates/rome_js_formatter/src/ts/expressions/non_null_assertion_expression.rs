use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsNonNullAssertionExpression;
use rome_js_syntax::TsNonNullAssertionExpressionFields;

impl FormatNodeFields<TsNonNullAssertionExpression>
    for FormatNodeRule<TsNonNullAssertionExpression>
{
    fn format_fields(
        node: &TsNonNullAssertionExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNonNullAssertionExpressionFields {
            expression,
            excl_token,
        } = node.as_fields();

        formatted![formatter, [expression.format(), excl_token.format()]]
    }
}
